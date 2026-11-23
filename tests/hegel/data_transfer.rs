// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hegel::TestCase;
use hegel::generators as gs;
use pica::packets::uci::{
    AppConfigTlv, AppConfigTlvType, CreditAvailability, DataMessageRcv, DataRcvStatusCode,
    DataTransferNtfStatusCode, DeviceRole, DeviceType, MacAddressMode, SessionDataCreditNtf,
    SessionDataTransferStatusNtf, SessionDataTransferStatusNtfConfig, SessionInfoNtfConfig,
    SessionType,
};

use crate::common::{FixedEstimator, PicaHarness};
use crate::config::{
    SAMPLE_MEASUREMENT, encode_data_snd_fragments, twr_mac_pair, twr_session_tlvs,
};

#[hegel::test]
fn data_transfer_delivers_arbitrary_payload(tc: TestCase) {
    let session_id = tc.draw(gs::integers::<u32>().min_value(1));
    let seq_num = tc.draw(gs::integers::<u16>());
    let payload_len = tc.draw(gs::integers::<usize>().min_value(1).max_value(2500));
    let pattern = tc.draw(gs::vecs(gs::integers::<u8>()).min_size(1).max_size(64));
    let standalone = tc.draw(gs::booleans());
    let repetition_count = tc.draw(gs::integers::<u8>().max_value(2));
    let mac_mode = tc.draw(gs::sampled_from(vec![
        MacAddressMode::Mode0,
        MacAddressMode::Mode2,
    ]));
    let c0 = tc.draw(gs::integers::<u8>().min_value(1).max_value(127));
    let c1 = tc.draw(gs::integers::<u8>().min_value(1).max_value(127));
    let (controller_mac, controlee_mac) = twr_mac_pair(mac_mode, c0, c1);
    let controlee_addr = u64::from(controlee_mac);
    let controller_addr = u64::from(controller_mac);

    let session_type = if standalone {
        SessionType::FiraDataTransferSession
    } else {
        SessionType::FiraRangingAndInBandDataSession
    };

    let mut harness = PicaHarness::new(Box::new(FixedEstimator(SAMPLE_MEASUREMENT)));
    let controller = harness.add_reset_device();
    let controlee = harness.add_reset_device();

    harness.init_and_configure(
        controller,
        session_id,
        session_type,
        twr_session_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            mac_mode,
            controller_mac,
            controlee_mac,
            55,
            vec![
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                    v: vec![SessionInfoNtfConfig::Disable as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionDataTransferStatusNtfConfig,
                    v: vec![SessionDataTransferStatusNtfConfig::Enable as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::DataRepetitionCount,
                    v: vec![repetition_count],
                },
            ],
        ),
    );
    harness.init_and_configure(
        controlee,
        session_id,
        session_type,
        twr_session_tlvs(
            DeviceRole::Responder,
            DeviceType::Controlee,
            mac_mode,
            controlee_mac,
            controller_mac,
            55,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                v: vec![SessionInfoNtfConfig::Disable as u8],
            }],
        ),
    );

    harness.start_session(controlee, session_id);
    harness.start_session(controller, session_id);

    let payload: Vec<u8> = (0..payload_len)
        .map(|i| pattern.get(i % pattern.len()).copied().unwrap_or(0) ^ (i as u8))
        .collect();
    let fragments = encode_data_snd_fragments(session_id, controlee_addr, seq_num, payload.clone());
    for (idx, pkt) in fragments.iter().enumerate() {
        let is_last = idx + 1 == fragments.len();
        harness.pica.handle_packet(controller, pkt, harness.now);
        let credit = harness.pop::<SessionDataCreditNtf>(controller);
        assert_eq!(credit.session_token, session_id);
        assert_eq!(
            credit.credit_availability,
            if is_last {
                CreditAvailability::CreditNotAvailable
            } else {
                CreditAvailability::CreditAvailable
            }
        );
    }

    for tx in 1..=repetition_count {
        let rcv = harness.tick_until::<DataMessageRcv>(controlee);
        assert_eq!(rcv.session_handle, session_id);
        assert_eq!(rcv.status, DataRcvStatusCode::UciStatusSuccess);
        assert_eq!(rcv.source_address, controller_addr);
        assert_eq!(rcv.data_sequence_number, seq_num);
        assert_eq!(rcv.application_data, payload);

        let rep_ntf = harness.pop::<SessionDataTransferStatusNtf>(controller);
        assert_eq!(
            rep_ntf.status,
            DataTransferNtfStatusCode::UciDataTransferStatusRepetitionOk
        );
        assert_eq!(rep_ntf.tx_count, tx);
        assert_eq!(rep_ntf.uci_sequence_number, seq_num);
    }

    let rcv = harness.tick_until::<DataMessageRcv>(controlee);
    assert_eq!(rcv.session_handle, session_id);
    assert_eq!(rcv.status, DataRcvStatusCode::UciStatusSuccess);
    assert_eq!(rcv.source_address, controller_addr);
    assert_eq!(rcv.data_sequence_number, seq_num);
    assert_eq!(rcv.application_data, payload);

    assert_eq!(
        harness
            .pop::<SessionDataCreditNtf>(controller)
            .credit_availability,
        CreditAvailability::CreditAvailable
    );
    let status_ntf = harness.pop::<SessionDataTransferStatusNtf>(controller);
    assert_eq!(
        status_ntf.status,
        DataTransferNtfStatusCode::UciDataTransferStatusOk
    );
    assert_eq!(status_ntf.tx_count, repetition_count + 1);
    assert_eq!(status_ntf.uci_sequence_number, seq_num);

    harness.stop_and_deinit_session(controller, session_id);
    harness.stop_and_deinit_session(controlee, session_id);
}
