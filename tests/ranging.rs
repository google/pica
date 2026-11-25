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
use pica::packets::uci::{self, *};

const MULTICAST_SESSION_ID: u32 = 0x1003;
const MAX_MEAS_SESSION_ID: u32 = 0x1004;
const DL_TDOA_SESSION_ID: u32 = 0x1005;
const CCC_ALIRO_SESSION_ID: u32 = 0x1006;

const CONTROLLER_SHORT_MAC: [u8; 2] = [0x1A, 0x2B];
const CONTROLEE_SHORT_MAC: [u8; 2] = [0x3C, 0x4D];
const ANCHOR_SHORT_MAC: [u8; 2] = [0x34, 0x12];
const ANCHOR_SHORT_ADDR: u16 = 0x1234;
const DT_TAG_MAC: [u8; 2] = [0x78, 0x56];

const RANGING_DURATION_55_MS_LE: [u8; 4] = [0x37, 0x00, 0x00, 0x00];
const MAX_CONTROLEE_COUNT: u8 = 8;
const CONTROLEE_MAC_PREFIX: u8 = 0x50;
const SUBSESSION_ID_BASE: u32 = 100;
const SUBSESSION_KEY_SEED: u8 = 0xA0;
const SUBSESSION_ID_REMOVED: u32 = 0x0102_0304;
const MAX_MEASUREMENTS_1_LE: [u8; 2] = [0x01, 0x00];

const SAMPLE_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 175,
    azimuth: -10,
    elevation: 6,
};

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
    [
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
    ]
    .into_iter()
    .chain(extra)
    .collect()
}

#[test]
fn fragmented_multicast_command_adds_eight_controlees() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(SAMPLE_MEASUREMENT)));
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
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(SAMPLE_MEASUREMENT)));
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
    assert_eq!(m.distance, SAMPLE_MEASUREMENT.range);

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
fn dl_tdoa_tick_dispatches_session_info_ntf() {
    let mut harness = PicaHarness::new(Box::new(FixedEstimator(SAMPLE_MEASUREMENT)));
    harness
        .pica
        .create_anchor(pica::MacAddress::Short(ANCHOR_SHORT_MAC))
        .expect("create virtual anchor");
    let tag = harness.add_reset_device();

    harness.init_and_configure(
        tag,
        DL_TDOA_SESSION_ID,
        SessionType::FiraRangingSession,
        vec![
            AppConfigTlv {
                cfg_id: AppConfigTlvType::DeviceRole,
                v: vec![DeviceRole::DtTag as u8],
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::DeviceType,
                v: vec![DeviceType::Controlee as u8],
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::DeviceMacAddress,
                v: DT_TAG_MAC.to_vec(),
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::MultiNodeMode,
                v: vec![MultiNodeMode::OneToMany as u8],
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::ScheduleMode,
                v: vec![ScheduleMode::TimeScheduled as u8],
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
        ],
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
    harness.start_session(tag, DL_TDOA_SESSION_ID);

    let ntf = harness.tick_until::<ShortMacDlTDoASessionInfoNtf>(tag);
    assert_eq!(ntf.dl_tdoa_measurements.len(), 1);
    let m = ntf
        .dl_tdoa_measurements
        .first()
        .expect("expected DL-TDoA measurement");
    assert_eq!(m.mac_address, ANCHOR_SHORT_ADDR);
    assert_eq!(m.measurement.message_type, DlTdoaMessageType::PollDtm);

    harness.stop_and_deinit_session(tag, DL_TDOA_SESSION_ID);
}

#[test]
fn ccc_and_aliro_sessions_start_and_stop_cleanly() {
    for (session_type, extra) in [
        (SessionType::Ccc, None),
        (
            SessionType::Aliro,
            Some(AppConfigTlv {
                cfg_id: AppConfigTlvType::SessionKey,
                v: ALIRO_SESSION_KEY.to_vec(),
            }),
        ),
    ] {
        let mut harness = PicaHarness::new(Box::new(FixedEstimator(SAMPLE_MEASUREMENT)));
        let host = harness.add_reset_device();
        let tlvs: Vec<AppConfigTlv> = [
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
        .into_iter()
        .chain(extra)
        .collect();
        harness.init_and_configure(host, CCC_ALIRO_SESSION_ID, session_type, tlvs);
        harness.start_session(host, CCC_ALIRO_SESSION_ID);
        harness.stop_and_deinit_session(host, CCC_ALIRO_SESSION_ID);
    }
}
