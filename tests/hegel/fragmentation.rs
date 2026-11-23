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
use pdl_runtime::Packet;
use pica::packets::uci::{
    self, DeviceRole, DeviceType, HEADER_SIZE, MacAddressMode, SessionInitCmd, SessionInitRsp,
    SessionSetAppConfigCmd, SessionSetAppConfigRsp, SessionState, SessionStatusNtf, SessionType,
};
use pica::{Handle, MacAddress};

use crate::common::{FixedEstimator, PicaHarness};
use crate::config::{SAMPLE_MEASUREMENT, encode_control_fragments, twr_session_tlvs};

fn init_fira_harness(session_id: u32) -> (PicaHarness, Handle) {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(SAMPLE_MEASUREMENT)));
    let handle = harness.add_reset_device();
    harness.send_control(
        handle,
        SessionInitCmd {
            session_id,
            session_type: SessionType::FiraRangingSession,
        },
    );
    let init_rsp = harness.pop::<SessionInitRsp>(handle);
    assert_eq!(init_rsp.status, uci::Status::Ok);
    let init_ntf = harness.pop::<SessionStatusNtf>(handle);
    assert_eq!(init_ntf.session_state, SessionState::SessionStateInit);
    (harness, handle)
}

#[hegel::test]
fn fragmented_control_command_matches_unfragmented(tc: TestCase) {
    let session_id = tc.draw(gs::integers::<u32>().min_value(1));
    let mac0 = tc.draw(gs::integers::<u8>().min_value(1));
    let mac1 = tc.draw(gs::integers::<u8>().min_value(1));
    let duration_ms = tc.draw(gs::integers::<u32>().min_value(10).max_value(500));

    let cmd = SessionSetAppConfigCmd {
        session_token: session_id,
        tlvs: twr_session_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            MacAddressMode::Mode0,
            MacAddress::Short([mac0, mac1]),
            MacAddress::Short([mac0.wrapping_add(1), mac1.wrapping_add(1)]),
            duration_ms,
            vec![],
        ),
    };
    let payload_len = cmd
        .encode_to_vec()
        .expect("encode SessionSetAppConfigCmd")
        .len()
        .saturating_sub(HEADER_SIZE);
    let chunk_size = tc.draw(
        gs::integers::<usize>()
            .min_value(1)
            .max_value(payload_len.max(1)),
    );

    let (mut unfragmented, h_unfrag) = init_fira_harness(session_id);
    let (mut fragmented, h_frag) = init_fira_harness(session_id);
    let fragments = encode_control_fragments(&cmd, chunk_size);

    unfragmented.send_control(h_unfrag, cmd);
    let expected_rsp = unfragmented.pop::<SessionSetAppConfigRsp>(h_unfrag);
    assert_eq!(expected_rsp.status, uci::Status::Ok);
    let expected_ntf = unfragmented.pop::<SessionStatusNtf>(h_unfrag);
    assert_eq!(expected_ntf.session_state, SessionState::SessionStateIdle);

    for frag in fragments {
        fragmented.pica.handle_packet(h_frag, &frag, fragmented.now);
    }

    let actual_rsp = fragmented.pop::<SessionSetAppConfigRsp>(h_frag);
    let actual_ntf = fragmented.pop::<SessionStatusNtf>(h_frag);
    assert_eq!(actual_rsp.status, expected_rsp.status);
    assert_eq!(actual_rsp.cfg_status, expected_rsp.cfg_status);
    assert_eq!(actual_ntf.session_state, expected_ntf.session_state);
}
