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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use common::{FixedEstimator, PicaHarness};
use pdl_runtime::Packet;
use pica::packets::uci::{self, *};
use pica::{Handle, RangingEstimator, RangingMeasurement};

#[derive(Clone, Default)]
struct PairwiseEstimator {
    pairs: Arc<RwLock<HashMap<(Handle, Handle), RangingMeasurement>>>,
}

impl PairwiseEstimator {
    fn insert(&self, left: Handle, right: Handle, measurement: RangingMeasurement) {
        self.pairs
            .write()
            .expect("lock pairwise estimator")
            .insert((left, right), measurement);
    }
}

impl RangingEstimator for PairwiseEstimator {
    fn estimate(&self, left: &Handle, right: &Handle) -> Option<RangingMeasurement> {
        self.pairs
            .read()
            .expect("lock pairwise estimator")
            .get(&(*left, *right))
            .copied()
    }
}

const TWR_SHORT_SESSION_ID: u32 = 0x1001;
const TWR_EXT_SESSION_ID: u32 = 0x1002;
const MULTICAST_SESSION_ID: u32 = 0x1003;
const MAX_MEAS_SESSION_ID: u32 = 0x1004;
const DL_TDOA_SESSION_ID: u32 = 0x1005;
const CCC_ALIRO_SESSION_ID: u32 = 0x1006;

const CONTROLLER_SHORT_MAC: [u8; 2] = [0x1A, 0x2B];
const CONTROLEE_SHORT_MAC: [u8; 2] = [0x3C, 0x4D];
const CONTROLLER_EXT_MAC: [u8; 8] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
const CONTROLEE_EXT_MAC: [u8; 8] = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x12, 0x34];
const CONTROLEE_EXT_ADDR: u64 = 0x3412_FFEE_DDCC_BBAA;

const INIT_ANCHOR_MAC: [u8; 2] = [0x34, 0x12];
const RESP_ANCHOR_MAC: [u8; 2] = [0x56, 0x34];
const DT_TAG_MAC: [u8; 2] = [0x78, 0x56];

const RANGING_DURATION_55_MS_LE: [u8; 4] = [0x37, 0x00, 0x00, 0x00];
const MAX_CONTROLEE_COUNT: u8 = 8;
const CONTROLEE_MAC_PREFIX: u8 = 0x50;
const SUBSESSION_ID_BASE: u32 = 100;
const SUBSESSION_KEY_SEED: u8 = 0xA0;
const SUBSESSION_ID_REMOVED: u32 = 0x0102_0304;
const MAX_MEASUREMENTS_1_LE: [u8; 2] = [0x01, 0x00];

const TWR_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 150,
    azimuth: 12,
    elevation: -4,
};
const MULTICAST_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 200,
    azimuth: 15,
    elevation: -5,
};
const MAX_MEAS_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 175,
    azimuth: -10,
    elevation: 6,
};
const CCC_ALIRO_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 180,
    azimuth: 5,
    elevation: -2,
};
const TAG_TO_INIT_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 300,
    azimuth: 25,
    elevation: -12,
};
const INIT_TO_RESP_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 600,
    azimuth: 0,
    elevation: 0,
};
const TAG_TO_RESP_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 450,
    azimuth: -18,
    elevation: 8,
};

// Default slot duration is 2400 RSTU (127,795,200 ticks); ToF(300/450/600 cm) = 639/959/1279 ticks.
const ONE_SLOT_TICKS: u64 = 127_795_200;
const TAG_TO_INIT_TOF_TICKS: u64 = 639;
const TAG_TO_RESP_TOF_TICKS: u64 = 959;
const DS_TWR_ROUND_1_REPLY_TICKS: u32 = 127_793_921;

const ALIRO_SESSION_KEY: [u8; 16] = [
    0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF,
];

fn fira_twr_tlvs(
    role: DeviceRole,
    dev_type: DeviceType,
    mac_mode: MacAddressMode,
    local_mac: &[u8],
    dst_mac: &[u8],
    multi_node_mode: MultiNodeMode,
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
            cfg_id: AppConfigTlvType::MacAddressMode,
            v: vec![mac_mode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceMacAddress,
            v: local_mac.to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MultiNodeMode,
            v: vec![multi_node_mode as u8],
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

fn run_twr_lifecycle(
    session_id: u32,
    mac_mode: MacAddressMode,
    mac1: &[u8],
    mac2: &[u8],
) -> (PicaHarness, pica::Handle, pica::Handle) {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(TWR_MEASUREMENT)));
    let controller = harness.add_reset_device();
    let controlee = harness.add_reset_device();

    harness.init_and_configure(
        controller,
        session_id,
        SessionType::FiraRangingSession,
        fira_twr_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            mac_mode,
            mac1,
            mac2,
            MultiNodeMode::OneToOne,
            vec![],
        ),
    );
    harness.init_and_configure(
        controlee,
        session_id,
        SessionType::FiraRangingSession,
        fira_twr_tlvs(
            DeviceRole::Responder,
            DeviceType::Controlee,
            mac_mode,
            mac2,
            mac1,
            MultiNodeMode::OneToOne,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                v: vec![SessionInfoNtfConfig::Disable as u8],
            }],
        ),
    );

    harness.start_session(controlee, session_id);
    harness.start_session(controller, session_id);
    (harness, controller, controlee)
}

#[test]
fn twr_ranging_short_mac_lifecycle() {
    let (mut harness, controller, controlee) = run_twr_lifecycle(
        TWR_SHORT_SESSION_ID,
        MacAddressMode::Mode0,
        &CONTROLLER_SHORT_MAC,
        &CONTROLEE_SHORT_MAC,
    );
    let ntf = harness.tick_until::<ShortMacTwoWaySessionInfoNtf>(controller);
    let m = ntf
        .two_way_ranging_measurements
        .first()
        .expect("expected TWR measurement");
    assert_eq!(m.status, uci::Status::Ok);
    assert_eq!(m.distance, TWR_MEASUREMENT.range);

    harness.stop_and_deinit_session(controller, TWR_SHORT_SESSION_ID);
    harness.stop_and_deinit_session(controlee, TWR_SHORT_SESSION_ID);
}

#[test]
fn twr_ranging_extended_mac_lifecycle() {
    let (mut harness, controller, controlee) = run_twr_lifecycle(
        TWR_EXT_SESSION_ID,
        MacAddressMode::Mode2,
        &CONTROLLER_EXT_MAC,
        &CONTROLEE_EXT_MAC,
    );
    let ntf = harness.tick_until::<ExtendedMacTwoWaySessionInfoNtf>(controller);
    let m = ntf
        .two_way_ranging_measurements
        .first()
        .expect("expected TWR measurement");
    assert_eq!(m.status, uci::Status::Ok);
    assert_eq!(m.mac_address, CONTROLEE_EXT_ADDR);
    assert_eq!(m.distance, TWR_MEASUREMENT.range);

    harness.stop_and_deinit_session(controller, TWR_EXT_SESSION_ID);
    harness.stop_and_deinit_session(controlee, TWR_EXT_SESSION_ID);
}

#[test]
fn fragmented_multicast_command_adds_eight_controlees() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(MULTICAST_MEASUREMENT)));
    let controller = harness.add_reset_device();

    harness.init_and_configure(
        controller,
        MULTICAST_SESSION_ID,
        SessionType::FiraRangingSession,
        fira_twr_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::StsConfig,
                v: vec![StsConfig::ProvisionedForResponderSubSessionKey as u8],
            }],
        ),
    );

    harness.send_control(
        controller,
        SessionUpdateControllerMulticastListCmd {
            session_token: MULTICAST_SESSION_ID,
            action: UpdateMulticastListAction::RemoveControlee,
            payload: SessionUpdateControllerMulticastListCmdPayload {
                controlees: vec![Controlee {
                    short_address: CONTROLEE_SHORT_MAC,
                    subsession_id: SUBSESSION_ID_REMOVED,
                }],
            }
            .encode_to_vec()
            .expect("encode remove controlee"),
        },
    );
    assert_eq!(
        harness
            .pop::<SessionUpdateControllerMulticastListRsp>(controller)
            .status,
        uci::Status::Ok
    );

    // 8 * 38-byte controlees -> >255B payload, exercising PBF control packet reassembly.
    let controlees: Vec<_> = (0u8..MAX_CONTROLEE_COUNT)
        .map(|i| Controlee_V2_0_32_Byte_Version {
            short_address: [CONTROLEE_MAC_PREFIX, i + 1],
            subsession_id: u32::from(i) + SUBSESSION_ID_BASE,
            subsession_key: [SUBSESSION_KEY_SEED.wrapping_add(i); 32],
        })
        .collect();
    harness.send_control(
        controller,
        SessionUpdateControllerMulticastListCmd {
            session_token: MULTICAST_SESSION_ID,
            action: UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey,
            payload: SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload { controlees }
                .encode_to_vec()
                .expect("encode add controlees"),
        },
    );
    let rsp = harness.pop::<SessionUpdateControllerMulticastListRsp>(controller);
    assert_eq!(rsp.status, uci::Status::Ok);
    assert_eq!(rsp.controlee_status.len(), usize::from(MAX_CONTROLEE_COUNT));
}

#[test]
fn max_number_of_measurements_auto_stops_session() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(MAX_MEAS_MEASUREMENT)));
    let controller = harness.add_reset_device();
    let controlee = harness.add_reset_device();

    harness.init_and_configure(
        controlee,
        MAX_MEAS_SESSION_ID,
        SessionType::FiraRangingSession,
        fira_twr_tlvs(
            DeviceRole::Responder,
            DeviceType::Controlee,
            MacAddressMode::Mode0,
            &CONTROLEE_SHORT_MAC,
            &CONTROLLER_SHORT_MAC,
            MultiNodeMode::OneToOne,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                v: vec![SessionInfoNtfConfig::Disable as u8],
            }],
        ),
    );
    harness.init_and_configure(
        controller,
        MAX_MEAS_SESSION_ID,
        SessionType::FiraRangingSession,
        fira_twr_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            MultiNodeMode::OneToOne,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::MaxNumberOfMeasurements,
                v: MAX_MEASUREMENTS_1_LE.to_vec(),
            }],
        ),
    );

    harness.start_session(controlee, MAX_MEAS_SESSION_ID);
    harness.start_session(controller, MAX_MEAS_SESSION_ID);

    let ntf = harness.tick_until::<ShortMacTwoWaySessionInfoNtf>(controller);
    let m = ntf
        .two_way_ranging_measurements
        .first()
        .expect("expected TWR measurement");
    assert_eq!(m.status, uci::Status::Ok);
    assert_eq!(m.distance, MAX_MEAS_MEASUREMENT.range);

    let status = harness.pop::<SessionStatusNtf>(controller);
    assert_eq!(
        status.reason_code,
        ReasonCode::MaxNumberOfMeasurementsReached
    );
    assert_eq!(
        harness.pop::<CoreDeviceStatusNtf>(controller).device_state,
        DeviceState::DeviceStateReady
    );
}

#[test]
fn dl_tdoa_multi_anchor_end_to_end_via_tick() {
    let estimator = PairwiseEstimator::default();
    let mut harness = PicaHarness::new(Box::new(estimator.clone()));
    let init_anchor = harness.add_reset_device();
    let resp_anchor = harness.add_reset_device();
    let tag = harness.add_reset_device();
    estimator.insert(tag, init_anchor, TAG_TO_INIT_MEASUREMENT);
    estimator.insert(init_anchor, resp_anchor, INIT_TO_RESP_MEASUREMENT);
    estimator.insert(tag, resp_anchor, TAG_TO_RESP_MEASUREMENT);

    let base_dl_tdoa = |role: DeviceRole, dev_type: DeviceType, mac: [u8; 2]| {
        vec![
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
                v: mac.to_vec(),
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
                v: vec![RangingRoundUsage::OneWayRangingDlTdoa as u8],
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::RangingDuration,
                v: RANGING_DURATION_55_MS_LE.to_vec(),
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::MacAddressMode,
                v: vec![MacAddressMode::Mode0 as u8],
            },
        ]
    };

    harness.init_and_configure(
        init_anchor,
        DL_TDOA_SESSION_ID,
        SessionType::FiraRangingSession,
        base_dl_tdoa(
            DeviceRole::DtAnchor,
            DeviceType::Controller,
            INIT_ANCHOR_MAC,
        ),
    );
    harness.send_control(
        init_anchor,
        SessionUpdateDtAnchorRangingRoundsCmd {
            session_token: DL_TDOA_SESSION_ID,
            ranging_rounds: vec![DtAnchorRangingRound {
                round_index: 1,
                role: DtAnchorActingRole::Initiator,
            }],
        },
    );
    assert_eq!(
        harness
            .pop::<SessionUpdateDtAnchorRangingRoundsRsp>(init_anchor)
            .status,
        uci::Status::Ok
    );

    harness.init_and_configure(
        resp_anchor,
        DL_TDOA_SESSION_ID,
        SessionType::FiraRangingSession,
        base_dl_tdoa(
            DeviceRole::DtAnchor,
            DeviceType::Controller,
            RESP_ANCHOR_MAC,
        ),
    );
    harness.send_control(
        resp_anchor,
        SessionUpdateDtAnchorRangingRoundsCmd {
            session_token: DL_TDOA_SESSION_ID,
            ranging_rounds: vec![DtAnchorRangingRound {
                round_index: 1,
                role: DtAnchorActingRole::Responder,
            }],
        },
    );
    assert_eq!(
        harness
            .pop::<SessionUpdateDtAnchorRangingRoundsRsp>(resp_anchor)
            .status,
        uci::Status::Ok
    );

    harness.init_and_configure(
        tag,
        DL_TDOA_SESSION_ID,
        SessionType::FiraRangingSession,
        base_dl_tdoa(DeviceRole::DtTag, DeviceType::Controlee, DT_TAG_MAC),
    );
    harness.send_control(
        tag,
        SessionUpdateDtTagRangingRoundsCmd {
            session_token: DL_TDOA_SESSION_ID,
            ranging_round_indexes: vec![1],
        },
    );
    assert_eq!(
        harness
            .pop::<SessionUpdateDtTagRangingRoundsRsp>(tag)
            .status,
        uci::Status::Ok
    );

    harness.start_session(init_anchor, DL_TDOA_SESSION_ID);
    harness.start_session(resp_anchor, DL_TDOA_SESSION_ID);
    harness.start_session(tag, DL_TDOA_SESSION_ID);

    let ntf = harness.tick_until::<ShortMacDlTDoASessionInfoNtf>(tag);
    let types: Vec<_> = ntf
        .dl_tdoa_measurements
        .iter()
        .map(|m| m.measurement.message_type)
        .collect();
    assert_eq!(
        types,
        vec![
            DlTdoaMessageType::PollDtm,
            DlTdoaMessageType::ResponseDtm,
            DlTdoaMessageType::FinalDtm,
        ]
    );
    let poll = &ntf
        .dl_tdoa_measurements
        .first()
        .expect("expected PollDtm")
        .measurement;
    let resp = &ntf
        .dl_tdoa_measurements
        .get(1)
        .expect("expected ResponseDtm")
        .measurement;
    let final_m = &ntf
        .dl_tdoa_measurements
        .get(2)
        .expect("expected FinalDtm")
        .measurement;
    let poll_tx = poll.tx_timestamp_64.expect("expected poll TX timestamp");
    let poll_rx = poll.rx_timestamp_64.expect("expected poll RX timestamp");
    let resp_tx = resp
        .tx_timestamp_64
        .expect("expected response TX timestamp");
    let resp_rx = resp
        .rx_timestamp_64
        .expect("expected response RX timestamp");
    let final_tx = final_m
        .tx_timestamp_64
        .expect("expected final TX timestamp");
    let final_rx = final_m
        .rx_timestamp_64
        .expect("expected final RX timestamp");

    assert_eq!(poll_rx.checked_sub(poll_tx), Some(TAG_TO_INIT_TOF_TICKS));
    assert_eq!(resp_tx.checked_sub(poll_tx), Some(ONE_SLOT_TICKS));
    assert_eq!(resp_rx.checked_sub(resp_tx), Some(TAG_TO_RESP_TOF_TICKS));
    assert_eq!(final_tx.checked_sub(resp_tx), Some(ONE_SLOT_TICKS));
    assert_eq!(final_rx.checked_sub(final_tx), Some(TAG_TO_INIT_TOF_TICKS));
    assert_eq!(resp.responder_reply_time, DS_TWR_ROUND_1_REPLY_TICKS);

    harness.stop_and_deinit_session(tag, DL_TDOA_SESSION_ID);
    harness.stop_and_deinit_session(resp_anchor, DL_TDOA_SESSION_ID);
    harness.stop_and_deinit_session(init_anchor, DL_TDOA_SESSION_ID);
}

fn ccc_aliro_base_tlvs() -> Vec<AppConfigTlv> {
    vec![
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceType,
            v: vec![DeviceType::Controller as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceRole,
            v: vec![DeviceRole::Initiator as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MultiNodeMode,
            v: vec![MultiNodeMode::OneToMany as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::HoppingMode,
            v: vec![CccHoppingMode::ExtensionContinuousAes as u8],
        },
    ]
}

#[test]
fn ccc_session_starts_and_stops_cleanly() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(CCC_ALIRO_MEASUREMENT)));
    let host = harness.add_reset_device();
    harness.init_and_configure(
        host,
        CCC_ALIRO_SESSION_ID,
        SessionType::Ccc,
        ccc_aliro_base_tlvs(),
    );
    harness.start_session(host, CCC_ALIRO_SESSION_ID);
    harness.stop_and_deinit_session(host, CCC_ALIRO_SESSION_ID);
}

#[test]
fn aliro_session_starts_and_stops_cleanly() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(CCC_ALIRO_MEASUREMENT)));
    let host = harness.add_reset_device();
    let mut tlvs = ccc_aliro_base_tlvs();
    tlvs.push(AppConfigTlv {
        cfg_id: AppConfigTlvType::SessionKey,
        v: ALIRO_SESSION_KEY.to_vec(),
    });
    harness.init_and_configure(host, CCC_ALIRO_SESSION_ID, SessionType::Aliro, tlvs);
    harness.start_session(host, CCC_ALIRO_SESSION_ID);
    harness.stop_and_deinit_session(host, CCC_ALIRO_SESSION_ID);
}
