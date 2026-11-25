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

mod common;

use common::{FixedEstimator, PicaHarness};
use pdl_runtime::Packet;
use pica::RangingMeasurement;
use pica::packets::uci::*;

const IN_BAND_SESSION_ID: u32 = 0x2001;
const STANDALONE_DATA_SESSION_ID: u32 = 0x2002;
const CONTROLLER_MAC: [u8; 2] = [0x34, 0x12];
const CONTROLLER_SHORT_ADDR: u64 = 0x1234;
const CONTROLEE_MAC: [u8; 2] = [0x78, 0x56];
const CONTROLEE_SHORT_ADDR: u64 = 0x5678;
const RANGING_DURATION_55_MS_LE: [u8; 4] = [0x37, 0x00, 0x00, 0x00];
const DATA_SAMPLE_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 275,
    azimuth: 18,
    elevation: -9,
};
const FRAGMENTED_PAYLOAD_LEN: u16 = 2500;
const FRAGMENTED_PATTERN_MOD: u16 = 251;

fn data_session_tlvs(
    role: DeviceRole,
    dev_type: DeviceType,
    local_mac: [u8; 2],
    dst_mac: [u8; 2],
    extra: Vec<AppConfigTlv>,
) -> Vec<AppConfigTlv> {
    let mut tlvs = vec![
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceRole,
            v: vec![role as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceType,
            v: vec![dev_type as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceMacAddress,
            v: local_mac.to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MacAddressMode,
            v: vec![MacAddressMode::Mode0 as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MultiNodeMode,
            v: vec![MultiNodeMode::OneToOne as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::ScheduleMode,
            v: vec![ScheduleMode::ContentionBased as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingRoundUsage,
            v: vec![RangingRoundUsage::DsTwrDeferredMode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingDuration,
            v: RANGING_DURATION_55_MS_LE.to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::NumberOfControlees,
            v: vec![1],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DstMacAddress,
            v: dst_mac.to_vec(),
        },
    ];
    tlvs.extend(extra);
    tlvs
}

fn send_fragmented_data(
    harness: &mut PicaHarness,
    handle: pica::Handle,
    session_id: u32,
    dst_address: u64,
    seq: u16,
    payload: &[u8],
) {
    let msg = DataMessageSnd {
        pbf: PacketBoundaryFlag::Complete,
        session_handle: session_id,
        destination_address: dst_address,
        data_sequence_number: seq,
        application_data: payload.to_vec(),
    }
    .encode_to_vec()
    .expect("encode DataMessageSnd");
    let raw_payload = msg.get(HEADER_SIZE..).expect("missing header");
    let chunks: Vec<_> = raw_payload.chunks(MAX_DATA_PACKET_PAYLOAD_SIZE).collect();

    for (idx, chunk) in chunks.iter().enumerate() {
        let is_last = idx + 1 == chunks.len();
        let pbf = if is_last {
            PacketBoundaryFlag::Complete
        } else {
            PacketBoundaryFlag::NotComplete
        };
        let pkt = DataPacket {
            dpf: DataPacketFormat::DataSnd,
            pbf,
            mt: MessageType::Data,
            payload: chunk.to_vec(),
        }
        .encode_to_vec()
        .expect("encode DataPacket");
        harness.pica.handle_packet(handle, &pkt, harness.now);
        let credit = harness.pop::<SessionDataCreditNtf>(handle);
        assert_eq!(
            credit.credit_availability,
            if is_last {
                CreditAvailability::CreditNotAvailable
            } else {
                CreditAvailability::CreditAvailable
            }
        );
    }
}

#[test]
fn in_band_fragmented_data_transfer_end_to_end() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(DATA_SAMPLE_MEASUREMENT)));
    let controller = harness.add_reset_device();
    let controlee = harness.add_reset_device();
    let payload: Vec<u8> = (0..FRAGMENTED_PAYLOAD_LEN)
        .map(|i| (i % FRAGMENTED_PATTERN_MOD) as u8)
        .collect();

    harness.init_and_configure(
        controller,
        IN_BAND_SESSION_ID,
        SessionType::FiraRangingAndInBandDataSession,
        data_session_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            CONTROLLER_MAC,
            CONTROLEE_MAC,
            vec![],
        ),
    );
    harness.init_and_configure(
        controlee,
        IN_BAND_SESSION_ID,
        SessionType::FiraRangingAndInBandDataSession,
        data_session_tlvs(
            DeviceRole::Responder,
            DeviceType::Controlee,
            CONTROLEE_MAC,
            CONTROLLER_MAC,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                v: vec![SessionInfoNtfConfig::Disable as u8],
            }],
        ),
    );

    harness.start_session(controlee, IN_BAND_SESSION_ID);
    harness.start_session(controller, IN_BAND_SESSION_ID);

    send_fragmented_data(
        &mut harness,
        controller,
        IN_BAND_SESSION_ID,
        CONTROLEE_SHORT_ADDR,
        1,
        &payload,
    );

    let credit = harness.tick_until::<SessionDataCreditNtf>(controller);
    assert_eq!(
        credit.credit_availability,
        CreditAvailability::CreditAvailable
    );
    let status = harness.pop::<SessionDataTransferStatusNtf>(controller);
    assert_eq!(
        status.status,
        DataTransferNtfStatusCode::UciDataTransferStatusOk
    );
    assert_eq!(status.tx_count, 1);
    let ranging_ntf = harness.pop::<ShortMacTwoWaySessionInfoNtf>(controller);
    let m = ranging_ntf
        .two_way_ranging_measurements
        .first()
        .expect("expected TWR measurement");
    assert_eq!(m.status, pica::packets::uci::Status::Ok);
    assert_eq!(m.distance, DATA_SAMPLE_MEASUREMENT.range);

    let data_rcv = harness.pop::<DataMessageRcv>(controlee);
    assert_eq!(data_rcv.application_data, payload);
    assert_eq!(data_rcv.source_address, CONTROLLER_SHORT_ADDR);

    harness.stop_and_deinit_session(controller, IN_BAND_SESSION_ID);
    harness.stop_and_deinit_session(controlee, IN_BAND_SESSION_ID);
}

#[test]
fn standalone_data_transfer_session_with_repetitions_end_to_end() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(DATA_SAMPLE_MEASUREMENT)));
    let controller = harness.add_reset_device();
    let controlee = harness.add_reset_device();

    harness.init_and_configure(
        controller,
        STANDALONE_DATA_SESSION_ID,
        SessionType::FiraDataTransferSession,
        data_session_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            CONTROLLER_MAC,
            CONTROLEE_MAC,
            vec![
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionDataTransferStatusNtfConfig,
                    v: vec![SessionDataTransferStatusNtfConfig::Enable as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::DataRepetitionCount,
                    v: vec![1],
                },
            ],
        ),
    );
    harness.init_and_configure(
        controlee,
        STANDALONE_DATA_SESSION_ID,
        SessionType::FiraDataTransferSession,
        data_session_tlvs(
            DeviceRole::Responder,
            DeviceType::Controlee,
            CONTROLEE_MAC,
            CONTROLLER_MAC,
            vec![],
        ),
    );

    harness.start_session(controlee, STANDALONE_DATA_SESSION_ID);
    harness.start_session(controller, STANDALONE_DATA_SESSION_ID);

    send_fragmented_data(
        &mut harness,
        controller,
        STANDALONE_DATA_SESSION_ID,
        CONTROLEE_SHORT_ADDR,
        1,
        b"repeated",
    );

    let rep_status = harness.tick_until::<SessionDataTransferStatusNtf>(controller);
    assert_eq!(
        rep_status.status,
        DataTransferNtfStatusCode::UciDataTransferStatusRepetitionOk
    );
    assert_eq!(rep_status.tx_count, 1);
    let rcv1 = harness.pop::<DataMessageRcv>(controlee);
    assert_eq!(rcv1.application_data, b"repeated");

    let credit = harness.tick_until::<SessionDataCreditNtf>(controller);
    assert_eq!(
        credit.credit_availability,
        CreditAvailability::CreditAvailable
    );
    let final_status = harness.pop::<SessionDataTransferStatusNtf>(controller);
    assert_eq!(
        final_status.status,
        DataTransferNtfStatusCode::UciDataTransferStatusOk
    );
    assert_eq!(final_status.tx_count, 2);
    let rcv2 = harness.pop::<DataMessageRcv>(controlee);
    assert_eq!(rcv2.application_data, b"repeated");

    harness.stop_and_deinit_session(controller, STANDALONE_DATA_SESSION_ID);
    harness.stop_and_deinit_session(controlee, STANDALONE_DATA_SESSION_ID);
}
