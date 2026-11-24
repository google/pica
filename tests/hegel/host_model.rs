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

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use hegel::TestCase;
use hegel::generators as gs;
use hegel::stateful::{Pool, machine, pool};
use pdl_runtime::Packet;
use pica::packets::uci::{
    self, AppConfigTlv, AppConfigTlvType, CapTlvType, ChannelNumber, ConfigParameter,
    ConfigParameterId, Controlee, ControleeStatus, CoreDeviceResetCmd, CoreGetCapsInfoCmd,
    CoreGetCapsInfoRsp, CoreGetConfigCmd, CoreGetConfigRsp, CoreGetDeviceInfoCmd,
    CoreGetDeviceInfoRsp, CoreSetConfigCmd, CoreSetConfigRsp, CreditAvailability,
    DataTransferNtfStatusCode, DeviceRole, DeviceType, DtAnchorActingRole, DtAnchorRangingRound,
    GroupId, MacAddressMode, MultiNodeMode, MulticastUpdateStatus, RangingRoundUsage, ReasonCode,
    ResetConfig, SessionDataCreditNtf, SessionDeinitCmd, SessionDeinitRsp, SessionGetAppConfigCmd,
    SessionGetAppConfigRsp, SessionGetCountCmd, SessionGetRangingCountCmd,
    SessionGetRangingCountRsp, SessionGetStateCmd, SessionGetStateRsp, SessionInitCmd,
    SessionInitRsp, SessionSetAppConfigCmd, SessionSetAppConfigRsp, SessionStartCmd,
    SessionStartRsp, SessionState, SessionStatusNtf, SessionStopCmd, SessionStopRsp, SessionType,
    SessionUpdateControllerMulticastListCmd, SessionUpdateControllerMulticastListCmdPayload,
    SessionUpdateControllerMulticastListRsp, SessionUpdateDtAnchorRangingRoundsCmd,
    SessionUpdateDtAnchorRangingRoundsRsp, SessionUpdateDtTagRangingRoundsCmd,
    SessionUpdateDtTagRangingRoundsRsp, Status, UpdateMulticastListAction,
};
use pica::{AlreadyExists, Handle, MacAddress, Pica, RangingMeasurement};

use crate::config::{
    PairwiseEstimator, SAMPLE_MEASUREMENT, TwrSessionSpec, dl_tdoa_tlvs, encode_data_snd_fragments,
    twr_spec_to_tlvs,
};
use crate::monitor::UciMonitor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SessionKind {
    Twr {
        is_controller: bool,
        session_type: SessionType,
        mac_mode: MacAddressMode,
        multi_node: MultiNodeMode,
        local_mac: MacAddress,
        controlees: [Option<MacAddress>; 8],
        controlee_len: usize,
        max_measurements: u16,
        peer_handle: Handle,
        peer_mac: MacAddress,
    },
    DlTdoaTag,
    CccOrAliro,
}

#[derive(Clone)]
struct PendingDataTransfer {
    seq_num: u16,
    payload: Vec<u8>,
    src_mac: MacAddress,
    peer_handle: Handle,
    expect_delivery: bool,
}

struct HostModel {
    pica: Pica,
    estimator: PairwiseEstimator,
    monitor: UciMonitor,
    now: Instant,
    devices: Pool<Handle>,
    anchors: Pool<(Handle, MacAddress)>,
    live_devices: BTreeSet<Handle>,
    active_anchors: BTreeMap<Handle, MacAddress>,
    seen_handles: BTreeSet<Handle>,
    sessions: BTreeMap<(Handle, u32), SessionKind>,
    in_flight_data: BTreeMap<(Handle, u32), PendingDataTransfer>,
    next_mac_suffix: u16,
}

impl HostModel {
    fn new(tc: &TestCase) -> Self {
        let estimator = PairwiseEstimator::default();
        Self {
            pica: Pica::new(Box::new(estimator.clone())),
            estimator,
            monitor: UciMonitor::default(),
            now: Instant::now(),
            devices: pool(tc),
            anchors: pool(tc),
            live_devices: BTreeSet::new(),
            active_anchors: BTreeMap::new(),
            seen_handles: BTreeSet::new(),
            sessions: BTreeMap::new(),
            in_flight_data: BTreeMap::new(),
            next_mac_suffix: 1,
        }
    }

    fn alloc_short_mac(&mut self) -> [u8; 2] {
        let id = self.next_mac_suffix;
        self.next_mac_suffix = self.next_mac_suffix.wrapping_add(1).max(1);
        id.to_le_bytes()
    }

    fn alloc_mac(&mut self, mode: MacAddressMode) -> MacAddress {
        let short = self.alloc_short_mac();
        match mode {
            MacAddressMode::Mode2 => {
                let [lo, hi] = short;
                MacAddress::Extended([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, lo, hi])
            }
            _ => MacAddress::Short(short),
        }
    }

    fn ensure_anchor(&mut self, mac: MacAddress) -> Handle {
        let existing = self
            .active_anchors
            .iter()
            .find_map(|(&h, &addr)| (addr == mac).then_some(h));

        match existing {
            Some(expected) => {
                assert_eq!(self.pica.create_anchor(mac), Err(AlreadyExists(expected)));
                expected
            }
            None => {
                let handle = self
                    .pica
                    .create_anchor(mac)
                    .expect("new anchor should succeed");
                assert!(
                    self.seen_handles.insert(handle),
                    "Handle {handle} was reused for anchor"
                );
                self.active_anchors.insert(handle, mac);
                self.anchors.add((handle, mac));
                handle
            }
        }
    }

    fn verify_core_info(&mut self, handle: Handle) {
        let info_pkts =
            self.monitor
                .send_control(&mut self.pica, handle, CoreGetDeviceInfoCmd {}, self.now);
        let info: CoreGetDeviceInfoRsp = UciMonitor::decode_response(&info_pkts, handle);
        assert_eq!(info.status, Status::Ok);

        let caps_pkts =
            self.monitor
                .send_control(&mut self.pica, handle, CoreGetCapsInfoCmd {}, self.now);
        let caps: CoreGetCapsInfoRsp = UciMonitor::decode_response(&caps_pkts, handle);
        assert_eq!(caps.status, Status::Ok);
        assert!(
            caps.tlvs
                .iter()
                .any(|t| t.t == CapTlvType::SupportedFiraPhyVersionRange && t.v.len() == 4)
        );
    }

    fn cleanup_device_sessions(&mut self, handle: Handle) {
        self.sessions.retain(|&(h, _), _| h != handle);
        self.in_flight_data.retain(|&(h, _), _| h != handle);
    }

    fn sync_session_presence(&mut self) {
        let monitor = &self.monitor;
        self.sessions
            .retain(|&(h, sid), _| monitor.has_session(h, sid));
        self.in_flight_data.retain(|&(h, sid), _| {
            monitor
                .device(h)
                .and_then(|d| d.sessions.get(&sid))
                .is_some_and(|s| s.state == SessionState::SessionStateActive)
        });
    }

    fn probe_unconfigured_fira_session(&mut self, handle: Handle, session_id: u32) {
        let init_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionInitCmd {
                session_id,
                session_type: SessionType::FiraRangingSession,
            },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionInitRsp>(&init_pkts, handle).status,
            Status::Ok
        );
        let start_init = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionStartCmd { session_id },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionStartRsp>(&start_init, handle).status,
            Status::ErrorSessionNotConfigured
        );
        let stop_init = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionStopCmd { session_id },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionStopRsp>(&stop_init, handle).status,
            Status::Rejected
        );
        let missing_mandatory = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionSetAppConfigCmd {
                session_token: session_id,
                tlvs: vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::DeviceRole,
                    v: vec![DeviceRole::Initiator as u8],
                }],
            },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionSetAppConfigRsp>(&missing_mandatory, handle)
                .status,
            Status::Rejected
        );

        let local_mac = self.alloc_mac(MacAddressMode::Mode0);
        let dst_mac = self.alloc_mac(MacAddressMode::Mode0);
        let short_dur_tlvs = twr_spec_to_tlvs(&TwrSessionSpec {
            role: DeviceRole::Initiator,
            dev_type: DeviceType::Controller,
            mac_mode: MacAddressMode::Mode0,
            multi_node_mode: MultiNodeMode::OneToOne,
            ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
            channel: ChannelNumber::ChannelNumber9,
            local_mac,
            dst_macs: vec![dst_mac],
            duration_ms: 10,
            max_measurements: 0,
            block_stride: 0,
        });
        let cfg_ok = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionSetAppConfigCmd {
                session_token: session_id,
                tlvs: short_dur_tlvs,
            },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionSetAppConfigRsp>(&cfg_ok, handle).status,
            Status::Ok
        );
        let bad_start = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionStartCmd { session_id },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionStartRsp>(&bad_start, handle).status,
            Status::Ok
        );
        let err_ntf: SessionStatusNtf = UciMonitor::decode_response(&bad_start, handle);
        assert_eq!(err_ntf.session_state, SessionState::SessionStateIdle);
        assert_eq!(err_ntf.reason_code, ReasonCode::ErrorInvalidRangingDuration);
        let deinit_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionDeinitCmd {
                session_token: session_id,
            },
            self.now,
        );
        assert_eq!(
            UciMonitor::decode_response::<SessionDeinitRsp>(&deinit_pkts, handle).status,
            Status::Ok
        );
    }
}

#[hegel::state_machine]
impl HostModel {
    #[rule(weight = 1.5)]
    fn add_device(&mut self, tc: TestCase) {
        let handle = self.pica.add_device();
        assert!(
            self.seen_handles.insert(handle),
            "Handle {handle} was reused across devices/anchors"
        );
        self.live_devices.insert(handle);
        self.monitor.register_device(handle);
        let _ = self.monitor.observe_drain(&mut self.pica);

        if tc.draw(gs::booleans()) {
            let probe = self.monitor.send_control(
                &mut self.pica,
                handle,
                CoreGetDeviceInfoCmd {},
                self.now,
            );
            assert!(
                probe.iter().any(|(h, bytes)| *h == handle
                    && UciMonitor::contains_group_id(bytes, GroupId::Core))
            );
        }

        let reset_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
            self.now,
        );
        let reset_rsp: uci::CoreDeviceResetRsp = UciMonitor::decode_response(&reset_pkts, handle);
        assert_eq!(reset_rsp.status, Status::Ok);
        self.verify_core_info(handle);
        self.devices.add(handle);
    }

    #[rule(weight = 0.4)]
    fn reset_live_device(&mut self, tc: TestCase) {
        let handle = *tc.draw(self.devices.values_reusable());
        let pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
            self.now,
        );
        let rsp: uci::CoreDeviceResetRsp = UciMonitor::decode_response(&pkts, handle);
        assert_eq!(rsp.status, Status::Ok);
        self.cleanup_device_sessions(handle);
        self.verify_core_info(handle);
    }

    #[rule(weight = 0.4)]
    fn remove_device(&mut self, tc: TestCase) {
        let handle = tc.draw(self.devices.values_consumed());
        assert!(self.pica.remove_device(handle));
        assert!(!self.pica.remove_device(handle));
        self.live_devices.remove(&handle);
        self.monitor.remove_device(handle);
        self.cleanup_device_sessions(handle);
        let remaining = self.monitor.observe_drain(&mut self.pica);
        assert!(
            remaining.iter().all(|(pkt_h, _)| *pkt_h != handle),
            "removed device {handle} left queued outbound packets"
        );
    }

    #[rule]
    fn create_anchor(&mut self, tc: TestCase) {
        let b0 = tc.draw(gs::integers::<u8>().min_value(1).max_value(8)) | 0xF0;
        let b1 = tc.draw(gs::integers::<u8>().min_value(1).max_value(8));
        let _ = self.ensure_anchor(MacAddress::Short([b0, b1]));
    }

    #[rule(weight = 0.4)]
    fn destroy_anchor(&mut self, tc: TestCase) {
        let (handle, _) = tc.draw(self.anchors.values_consumed());
        assert!(self.pica.destroy_anchor(handle));
        assert!(!self.pica.destroy_anchor(handle));
        self.active_anchors.remove(&handle);
    }

    #[rule(weight = 2.0)]
    fn create_and_start_paired_twr_session(&mut self, tc: TestCase) {
        tc.assume(self.live_devices.len() >= 2);
        let controller = *tc.draw(self.devices.values_reusable());
        let other_devices: Vec<Handle> = self
            .live_devices
            .iter()
            .copied()
            .filter(|&h| h != controller)
            .collect();
        let controlee = tc.draw(gs::sampled_from(other_devices));
        let session_id = tc.draw(gs::integers::<u32>().min_value(1).max_value(4));
        tc.assume(
            !self.monitor.has_session(controller, session_id)
                && !self.monitor.has_session(controlee, session_id),
        );

        let session_type = tc.draw(gs::sampled_from(vec![
            SessionType::FiraRangingSession,
            SessionType::FiraRangingAndInBandDataSession,
            SessionType::FiraDataTransferSession,
        ]));
        let mac_mode = tc.draw(gs::sampled_from(vec![
            MacAddressMode::Mode0,
            MacAddressMode::Mode2,
        ]));
        let multi_node_mode = tc.draw(gs::sampled_from(vec![
            MultiNodeMode::OneToOne,
            MultiNodeMode::OneToMany,
        ]));
        let ranging_round_usage = tc.draw(gs::sampled_from(vec![
            RangingRoundUsage::SsTwrDeferredMode,
            RangingRoundUsage::DsTwrDeferredMode,
        ]));
        let channel = tc.draw(gs::sampled_from(vec![
            ChannelNumber::ChannelNumber5,
            ChannelNumber::ChannelNumber9,
        ]));
        let max_measurements = tc.draw(gs::sampled_from(vec![0u16, 1, 2]));
        let block_stride = tc.draw(gs::sampled_from(vec![0u8, 1]));
        let duration_ms = tc.draw(gs::sampled_from(vec![55u32, 100, 200]));
        let range = tc.draw(gs::integers::<u16>().min_value(1).max_value(2000));

        if tc.draw(gs::booleans()) {
            self.estimator.insert(
                controller,
                controlee,
                RangingMeasurement {
                    range,
                    azimuth: 15,
                    elevation: -6,
                },
            );
            self.estimator.insert(
                controlee,
                controller,
                RangingMeasurement {
                    range,
                    azimuth: -15,
                    elevation: 6,
                },
            );
        } else {
            self.estimator.remove(controller, controlee);
            self.estimator.remove(controlee, controller);
        }

        let controller_mac = self.alloc_mac(mac_mode);
        let controlee_mac = self.alloc_mac(mac_mode);

        for (dev, role, dev_type, local_mac, dst_mac) in [
            (
                controller,
                DeviceRole::Initiator,
                DeviceType::Controller,
                controller_mac,
                controlee_mac,
            ),
            (
                controlee,
                DeviceRole::Responder,
                DeviceType::Controlee,
                controlee_mac,
                controller_mac,
            ),
        ] {
            let init_pkts = self.monitor.send_control(
                &mut self.pica,
                dev,
                SessionInitCmd {
                    session_id,
                    session_type,
                },
                self.now,
            );
            let init_rsp: SessionInitRsp = UciMonitor::decode_response(&init_pkts, dev);
            assert_eq!(init_rsp.status, Status::Ok);

            let status_ntf_extra = (session_type != SessionType::FiraRangingSession
                && role == DeviceRole::Initiator)
                .then_some(AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionDataTransferStatusNtfConfig,
                    v: vec![uci::SessionDataTransferStatusNtfConfig::Enable as u8],
                });
            let tlvs: Vec<AppConfigTlv> = twr_spec_to_tlvs(&TwrSessionSpec {
                role,
                dev_type,
                mac_mode,
                multi_node_mode,
                ranging_round_usage,
                channel,
                local_mac,
                dst_macs: vec![dst_mac],
                duration_ms,
                max_measurements,
                block_stride,
            })
            .into_iter()
            .chain(status_ntf_extra)
            .collect();
            let cfg_pkts = self.monitor.send_control(
                &mut self.pica,
                dev,
                SessionSetAppConfigCmd {
                    session_token: session_id,
                    tlvs,
                },
                self.now,
            );
            let cfg_rsp: SessionSetAppConfigRsp = UciMonitor::decode_response(&cfg_pkts, dev);
            assert_eq!(cfg_rsp.status, Status::Ok);
            self.monitor
                .set_session_max_measurements(dev, session_id, max_measurements);

            let get_pkts = self.monitor.send_control(
                &mut self.pica,
                dev,
                SessionGetAppConfigCmd {
                    session_token: session_id,
                    app_cfg: vec![
                        AppConfigTlvType::ChannelNumber,
                        AppConfigTlvType::RangingDuration,
                    ],
                },
                self.now,
            );
            let get_rsp: SessionGetAppConfigRsp = UciMonitor::decode_response(&get_pkts, dev);
            assert_eq!(get_rsp.status, Status::Ok);
            assert_eq!(
                get_rsp.tlvs,
                vec![
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::ChannelNumber,
                        v: vec![channel as u8],
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::RangingDuration,
                        v: duration_ms.to_le_bytes().to_vec(),
                    },
                ]
            );
        }

        for dev in [controlee, controller] {
            let start_pkts = self.monitor.send_control(
                &mut self.pica,
                dev,
                SessionStartCmd { session_id },
                self.now,
            );
            let start_rsp: SessionStartRsp = UciMonitor::decode_response(&start_pkts, dev);
            assert_eq!(start_rsp.status, Status::Ok);
        }

        let mut controlees = [None; 8];
        if let Some(slot) = controlees.first_mut() {
            *slot = Some(controlee_mac);
        }
        self.sessions.insert(
            (controller, session_id),
            SessionKind::Twr {
                is_controller: true,
                session_type,
                mac_mode,
                multi_node: multi_node_mode,
                local_mac: controller_mac,
                controlees,
                controlee_len: 1,
                max_measurements,
                peer_handle: controlee,
                peer_mac: controlee_mac,
            },
        );
        let mut peer_list = [None; 8];
        if let Some(slot) = peer_list.first_mut() {
            *slot = Some(controller_mac);
        }
        self.sessions.insert(
            (controlee, session_id),
            SessionKind::Twr {
                is_controller: false,
                session_type,
                mac_mode,
                multi_node: multi_node_mode,
                local_mac: controlee_mac,
                controlees: peer_list,
                controlee_len: 1,
                max_measurements,
                peer_handle: controller,
                peer_mac: controller_mac,
            },
        );
    }

    #[rule(weight = 0.8)]
    fn send_in_band_data(&mut self, tc: TestCase) {
        self.sync_session_presence();
        let candidates: Vec<((Handle, u32), SessionKind)> = self
            .sessions
            .iter()
            .filter_map(|(&key, &kind)| {
                let SessionKind::Twr { .. } = kind else {
                    return None;
                };
                let is_active = self
                    .monitor
                    .device(key.0)
                    .and_then(|d| d.sessions.get(&key.1))
                    .is_some_and(|s| s.state == SessionState::SessionStateActive);
                (is_active && !self.in_flight_data.contains_key(&key)).then_some((key, kind))
            })
            .collect();
        tc.assume(!candidates.is_empty());

        let ((handle, session_id), kind) = tc.draw(gs::sampled_from(candidates));
        let SessionKind::Twr {
            is_controller,
            session_type,
            local_mac,
            controlees,
            controlee_len,
            peer_handle,
            ..
        } = kind
        else {
            return;
        };

        let dst_mac = controlees
            .get(..controlee_len)
            .and_then(|slice| slice.first())
            .and_then(|m| *m)
            .unwrap_or(MacAddress::Short([0x01, 0x02]));
        let seq_num = tc.draw(gs::integers::<u16>());
        let payload_len = tc.draw(gs::integers::<usize>().min_value(1).max_value(1200));
        let seed = tc.draw(gs::integers::<u8>());
        let payload: Vec<u8> = (0..payload_len)
            .map(|i| seed.wrapping_add((i % 251) as u8))
            .collect();
        let fragments =
            encode_data_snd_fragments(session_id, u64::from(dst_mac), seq_num, payload.clone());

        let supports_data = matches!(
            session_type,
            SessionType::FiraRangingAndInBandDataSession | SessionType::FiraDataTransferSession
        );
        if !supports_data || !is_controller {
            if let Some(first_frag) = fragments.first() {
                self.pica.handle_packet(handle, first_frag, self.now);
                let drained = self.monitor.observe_drain(&mut self.pica);
                let status_ntf: uci::SessionDataTransferStatusNtf =
                    UciMonitor::decode_response(&drained, handle);
                let expected = if !supports_data {
                    DataTransferNtfStatusCode::UciDataTransferStatusSessionTypeNotSupported
                } else {
                    DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected
                };
                assert_eq!(status_ntf.status, expected);
            }
            return;
        }

        for (idx, pkt) in fragments.iter().enumerate() {
            let is_last = idx + 1 == fragments.len();
            self.pica.handle_packet(handle, pkt, self.now);
            let drained = self.monitor.observe_drain(&mut self.pica);
            let credit: SessionDataCreditNtf = UciMonitor::decode_response(&drained, handle);
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

        let peer_ready = self
            .monitor
            .device(peer_handle)
            .and_then(|d| d.sessions.get(&session_id))
            .is_some_and(|s| s.state == SessionState::SessionStateActive)
            && matches!(
                self.sessions.get(&(peer_handle, session_id)),
                Some(SessionKind::Twr {
                    is_controller: false,
                    peer_handle: back_peer,
                    ..
                }) if *back_peer == handle
            );
        self.in_flight_data.insert(
            (handle, session_id),
            PendingDataTransfer {
                seq_num,
                payload,
                src_mac: local_mac,
                peer_handle,
                expect_delivery: peer_ready,
            },
        );
    }

    #[rule]
    fn update_multicast_list(&mut self, tc: TestCase) {
        let candidates: Vec<(Handle, u32)> = self
            .sessions
            .iter()
            .filter_map(|(&(h, sid), kind)| match kind {
                SessionKind::Twr {
                    is_controller: true,
                    multi_node: MultiNodeMode::OneToMany,
                    ..
                } if self.monitor.has_session(h, sid) => Some((h, sid)),
                _ => None,
            })
            .collect();
        tc.assume(!candidates.is_empty());
        let (handle, session_id) = tc.draw(gs::sampled_from(candidates));
        let action = tc.draw(gs::sampled_from(vec![
            UpdateMulticastListAction::AddControlee,
            UpdateMulticastListAction::RemoveControlee,
        ]));
        let batch_len = tc.draw(gs::integers::<usize>().min_value(1).max_value(3));

        let Some(SessionKind::Twr {
            mac_mode,
            mut controlees,
            mut controlee_len,
            ..
        }) = self.sessions.get(&(handle, session_id)).copied()
        else {
            return;
        };

        let is_active = self
            .monitor
            .device(handle)
            .and_then(|d| d.sessions.get(&session_id))
            .is_some_and(|s| s.state == SessionState::SessionStateActive);

        let batch: Vec<[u8; 2]> = (0..batch_len)
            .map(|_| {
                if mac_mode == MacAddressMode::Mode0 && controlee_len > 0 && tc.draw(gs::booleans())
                {
                    let existing: Vec<[u8; 2]> = controlees
                        .get(..controlee_len)
                        .unwrap_or(&[])
                        .iter()
                        .flatten()
                        .filter_map(|&mac| match mac {
                            MacAddress::Short(bytes) => Some(bytes),
                            MacAddress::Extended(_) => None,
                        })
                        .collect();
                    match existing.as_slice() {
                        [only] => *only,
                        [] => self.alloc_short_mac(),
                        _ => tc.draw(gs::sampled_from(existing)),
                    }
                } else {
                    self.alloc_short_mac()
                }
            })
            .collect();

        let payload = SessionUpdateControllerMulticastListCmdPayload {
            controlees: batch
                .iter()
                .map(|&short_address| Controlee {
                    short_address,
                    subsession_id: 0,
                })
                .collect(),
        }
        .encode_to_vec()
        .expect("encode multicast payload");
        let pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionUpdateControllerMulticastListCmd {
                session_token: session_id,
                action,
                payload,
            },
            self.now,
        );
        let rsp: SessionUpdateControllerMulticastListRsp =
            UciMonitor::decode_response(&pkts, handle);

        if mac_mode != MacAddressMode::Mode0 {
            assert_eq!(rsp.status, Status::Rejected);
            assert!(rsp.controlee_status.is_empty());
            return;
        }

        let mut expected_status = Status::Ok;
        let mut active_removed_ntf = Vec::new();
        let expected_rsp: Vec<ControleeStatus> = batch
            .iter()
            .map(|&short_bytes| {
                let mac = MacAddress::Short(short_bytes);
                let present = controlees
                    .get(..controlee_len)
                    .unwrap_or(&[])
                    .iter()
                    .flatten()
                    .any(|&m| m == mac);
                let status = match action {
                    UpdateMulticastListAction::AddControlee => {
                        if present {
                            expected_status = Status::Failed;
                            MulticastUpdateStatus::ErrorAddressAlreadyPresent
                        } else if controlee_len >= 8 {
                            expected_status = Status::ErrorMulticastListFull;
                            MulticastUpdateStatus::ErrorMulticastListFull
                        } else {
                            if let Some(slot) = controlees.get_mut(controlee_len) {
                                *slot = Some(mac);
                                controlee_len += 1;
                            }
                            MulticastUpdateStatus::OkMulticastListUpdate
                        }
                    }
                    other => {
                        assert_eq!(other, UpdateMulticastListAction::RemoveControlee);
                        if !present {
                            expected_status = Status::Failed;
                            MulticastUpdateStatus::ErrorAddressNotFound
                        } else {
                            let mut kept = [None; 8];
                            let mut new_len = 0usize;
                            for &item in controlees.get(..controlee_len).unwrap_or(&[]) {
                                if item.is_some_and(|m| m != mac)
                                    && let Some(slot) = kept.get_mut(new_len)
                                {
                                    *slot = item;
                                    new_len += 1;
                                }
                            }
                            controlees = kept;
                            controlee_len = new_len;
                            if is_active {
                                active_removed_ntf.push(ControleeStatus {
                                    mac_address: short_bytes,
                                    status: MulticastUpdateStatus::OkMulticastListUpdate,
                                });
                            }
                            MulticastUpdateStatus::OkMulticastListUpdate
                        }
                    }
                };
                ControleeStatus {
                    mac_address: short_bytes,
                    status,
                }
            })
            .collect();

        assert_eq!(rsp.status, expected_status);
        assert_eq!(rsp.controlee_status, expected_rsp);

        if !active_removed_ntf.is_empty() {
            let ntf = pkts
                .iter()
                .filter_map(|(h, bytes)| {
                    (*h == handle)
                        .then(|| UciMonitor::decode_multicast_ntf(bytes))
                        .flatten()
                })
                .next()
                .expect("expected multicast NTF when removing active controlee");
            assert_eq!(ntf.session_token, session_id);
            assert_eq!(ntf.controlee_status, active_removed_ntf);
        }

        if let Some(SessionKind::Twr {
            controlees: saved_list,
            controlee_len: saved_len,
            ..
        }) = self.sessions.get_mut(&(handle, session_id))
        {
            *saved_list = controlees;
            *saved_len = controlee_len;
        }
        self.sync_session_presence();
    }

    #[rule]
    fn start_dl_tdoa_tag_session(&mut self, tc: TestCase) {
        let handle = *tc.draw(self.devices.values_reusable());
        let session_id = tc.draw(gs::integers::<u32>().min_value(10).max_value(12));
        tc.assume(!self.monitor.has_session(handle, session_id));

        let anchor_mac = self.alloc_mac(MacAddressMode::Mode0);
        let anchor_handle = self.ensure_anchor(anchor_mac);
        self.estimator
            .insert(handle, anchor_handle, SAMPLE_MEASUREMENT);

        let init_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionInitCmd {
                session_id,
                session_type: SessionType::FiraRangingSession,
            },
            self.now,
        );
        let init_rsp: SessionInitRsp = UciMonitor::decode_response(&init_pkts, handle);
        assert_eq!(init_rsp.status, Status::Ok);

        let tag_mac = self.alloc_mac(MacAddressMode::Mode0);
        let cfg_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionSetAppConfigCmd {
                session_token: session_id,
                tlvs: dl_tdoa_tlvs(
                    DeviceRole::DtTag,
                    MacAddressMode::Mode0,
                    ChannelNumber::ChannelNumber9,
                    tag_mac,
                    100,
                    1,
                    0,
                ),
            },
            self.now,
        );
        let cfg_rsp: SessionSetAppConfigRsp = UciMonitor::decode_response(&cfg_pkts, handle);
        assert_eq!(cfg_rsp.status, Status::Ok);
        self.monitor
            .set_session_max_measurements(handle, session_id, 1);

        let rounds_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionUpdateDtTagRangingRoundsCmd {
                session_token: session_id,
                ranging_round_indexes: vec![0, 1],
            },
            self.now,
        );
        let rounds_rsp: SessionUpdateDtTagRangingRoundsRsp =
            UciMonitor::decode_response(&rounds_pkts, handle);
        assert_eq!(rounds_rsp.status, Status::Ok);

        let start_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            SessionStartCmd { session_id },
            self.now,
        );
        let start_rsp: SessionStartRsp = UciMonitor::decode_response(&start_pkts, handle);
        assert_eq!(start_rsp.status, Status::Ok);

        self.sessions
            .insert((handle, session_id), SessionKind::DlTdoaTag);
    }

    #[rule(weight = 0.6)]
    fn ccc_aliro_or_dt_anchor_lifecycle(&mut self, tc: TestCase) {
        let handle = *tc.draw(self.devices.values_reusable());
        let session_id = tc.draw(gs::integers::<u32>().min_value(20).max_value(24));
        tc.assume(!self.monitor.has_session(handle, session_id));

        if tc.draw(gs::booleans()) {
            let session_type =
                tc.draw(gs::sampled_from(vec![SessionType::Ccc, SessionType::Aliro]));
            let init_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionInitCmd {
                    session_id,
                    session_type,
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionInitRsp>(&init_pkts, handle).status,
                Status::Ok
            );
            let cfg_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionSetAppConfigCmd {
                    session_token: session_id,
                    tlvs: vec![
                        AppConfigTlv {
                            cfg_id: AppConfigTlvType::DeviceRole,
                            v: vec![DeviceRole::Initiator as u8],
                        },
                        AppConfigTlv {
                            cfg_id: AppConfigTlvType::MultiNodeMode,
                            v: vec![MultiNodeMode::OneToOne as u8],
                        },
                    ],
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionSetAppConfigRsp>(&cfg_pkts, handle).status,
                Status::Ok
            );
            let start_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionStartCmd { session_id },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionStartRsp>(&start_pkts, handle).status,
                Status::Ok
            );
            self.sessions
                .insert((handle, session_id), SessionKind::CccOrAliro);
        } else {
            let init_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionInitCmd {
                    session_id,
                    session_type: SessionType::FiraRangingSession,
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionInitRsp>(&init_pkts, handle).status,
                Status::Ok
            );
            let anchor_mac = self.alloc_mac(MacAddressMode::Mode0);
            let tlvs: Vec<AppConfigTlv> = dl_tdoa_tlvs(
                DeviceRole::DtAnchor,
                MacAddressMode::Mode0,
                ChannelNumber::ChannelNumber9,
                anchor_mac,
                100,
                0,
                0,
            )
            .into_iter()
            .chain([AppConfigTlv {
                cfg_id: AppConfigTlvType::DlTdoaRangingMethod,
                v: vec![uci::DlTdoaRangingMethod::DsTwr as u8],
            }])
            .collect();
            let cfg_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionSetAppConfigCmd {
                    session_token: session_id,
                    tlvs,
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionSetAppConfigRsp>(&cfg_pkts, handle).status,
                Status::Ok
            );
            let empty_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionUpdateDtAnchorRangingRoundsCmd {
                    session_token: session_id,
                    ranging_rounds: vec![],
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionUpdateDtAnchorRangingRoundsRsp>(
                    &empty_pkts,
                    handle
                )
                .status,
                Status::InvalidParam
            );
            let valid_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionUpdateDtAnchorRangingRoundsCmd {
                    session_token: session_id,
                    ranging_rounds: vec![DtAnchorRangingRound {
                        round_index: 0,
                        role: DtAnchorActingRole::Initiator,
                    }],
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionUpdateDtAnchorRangingRoundsRsp>(
                    &valid_pkts,
                    handle
                )
                .status,
                Status::Ok
            );
            let deinit_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionDeinitCmd {
                    session_token: session_id,
                },
                self.now,
            );
            assert_eq!(
                UciMonitor::decode_response::<SessionDeinitRsp>(&deinit_pkts, handle).status,
                Status::Ok
            );
        }
    }

    #[rule]
    fn reconfigure_or_query(&mut self, tc: TestCase) {
        let handle = *tc.draw(self.devices.values_reusable());
        let active_ids = self.monitor.active_session_ids(handle);
        if let Some(&session_id) = active_ids.first() {
            if tc.draw(gs::booleans()) {
                let stride = tc.draw(gs::integers::<u8>().min_value(0).max_value(2));
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionSetAppConfigCmd {
                        session_token: session_id,
                        tlvs: vec![AppConfigTlv {
                            cfg_id: AppConfigTlvType::BlockStrideLength,
                            v: vec![stride],
                        }],
                    },
                    self.now,
                );
                let rsp: SessionSetAppConfigRsp = UciMonitor::decode_response(&pkts, handle);
                assert_eq!(rsp.status, Status::Ok);
            } else {
                // AoaResultReq is immutable while ACTIVE.
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionSetAppConfigCmd {
                        session_token: session_id,
                        tlvs: vec![AppConfigTlv {
                            cfg_id: AppConfigTlvType::AoaResultReq,
                            v: vec![uci::AoaResultReq::AoaDisabled as u8],
                        }],
                    },
                    self.now,
                );
                let rsp: SessionSetAppConfigRsp = UciMonitor::decode_response(&pkts, handle);
                assert_eq!(rsp.status, Status::ErrorSessionActive);
            }
            // CoreSetConfig is rejected while device is ACTIVE.
            let core_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                CoreSetConfigCmd {
                    parameters: vec![ConfigParameter {
                        id: ConfigParameterId::LowPowerMode,
                        value: vec![0],
                    }],
                },
                self.now,
            );
            let core_rsp: CoreSetConfigRsp = UciMonitor::decode_response(&core_pkts, handle);
            assert_eq!(core_rsp.status, Status::Rejected);
        } else {
            let low_power = tc.draw(gs::integers::<u8>().min_value(0).max_value(1));
            let core_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                CoreSetConfigCmd {
                    parameters: vec![ConfigParameter {
                        id: ConfigParameterId::LowPowerMode,
                        value: vec![low_power],
                    }],
                },
                self.now,
            );
            let core_rsp: CoreSetConfigRsp = UciMonitor::decode_response(&core_pkts, handle);
            assert_eq!(core_rsp.status, Status::Ok);
        }
    }

    #[rule]
    fn stop_or_deinit_session(&mut self, tc: TestCase) {
        self.sync_session_presence();
        let keys: Vec<(Handle, u32)> = self.sessions.keys().copied().collect();
        tc.assume(!keys.is_empty());
        let (handle, session_id) = tc.draw(gs::sampled_from(keys));
        let is_active = self
            .monitor
            .device(handle)
            .and_then(|d| d.sessions.get(&session_id))
            .is_some_and(|s| s.state == SessionState::SessionStateActive);

        if is_active && tc.draw(gs::booleans()) {
            let pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionStopCmd { session_id },
                self.now,
            );
            let rsp: SessionStopRsp = UciMonitor::decode_response(&pkts, handle);
            assert_eq!(rsp.status, Status::Ok);
        } else {
            let pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionDeinitCmd {
                    session_token: session_id,
                },
                self.now,
            );
            let rsp: SessionDeinitRsp = UciMonitor::decode_response(&pkts, handle);
            assert_eq!(rsp.status, Status::Ok);
            self.sessions.remove(&(handle, session_id));
        }
        self.sync_session_presence();
    }

    #[rule(weight = 0.9)]
    fn negative_lifecycle_and_config_probe(&mut self, tc: TestCase) {
        let handle = *tc.draw(self.devices.values_reusable());
        let missing_id = 0xDEAD_BEEF;
        match tc.draw(gs::integers::<u8>().min_value(0).max_value(7)) {
            0 => {
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionStartCmd {
                        session_id: missing_id,
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionStartRsp>(&pkts, handle).status,
                    Status::ErrorSessionNotExist
                );
            }
            1 => {
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionStopCmd {
                        session_id: missing_id,
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionStopRsp>(&pkts, handle).status,
                    Status::ErrorSessionNotExist
                );
            }
            2 => {
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionGetStateCmd {
                        session_token: missing_id,
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionGetStateRsp>(&pkts, handle).status,
                    Status::ErrorSessionNotExist
                );
            }
            3 => {
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionUpdateDtTagRangingRoundsCmd {
                        session_token: missing_id,
                        ranging_round_indexes: vec![0],
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionUpdateDtTagRangingRoundsRsp>(
                        &pkts, handle
                    )
                    .status,
                    Status::ErrorSessionNotExist
                );
            }
            4 => {
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionUpdateDtAnchorRangingRoundsCmd {
                        session_token: missing_id,
                        ranging_rounds: vec![DtAnchorRangingRound {
                            round_index: 0,
                            role: DtAnchorActingRole::Initiator,
                        }],
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionUpdateDtAnchorRangingRoundsRsp>(
                        &pkts, handle
                    )
                    .status,
                    Status::ErrorSessionNotExist
                );
            }
            5 => {
                let dev_sessions: Vec<u32> = self
                    .monitor
                    .device(handle)
                    .map(|d| d.sessions.keys().copied().collect())
                    .unwrap_or_default();
                tc.assume(!dev_sessions.is_empty());
                let dup_id = tc.draw(gs::sampled_from(dev_sessions));
                let pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionInitCmd {
                        session_id: dup_id,
                        session_type: SessionType::FiraRangingSession,
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionInitRsp>(&pkts, handle).status,
                    Status::ErrorSessionDuplicate
                );
            }
            6 => {
                let temp_id = 0x7000_0001;
                tc.assume(!self.monitor.has_session(handle, temp_id));
                let init_pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionInitCmd {
                        session_id: temp_id,
                        session_type: SessionType::DeviceTestMode,
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionInitRsp>(&init_pkts, handle).status,
                    Status::Ok
                );
                let cfg_pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionSetAppConfigCmd {
                        session_token: temp_id,
                        tlvs: vec![],
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionSetAppConfigRsp>(&cfg_pkts, handle).status,
                    Status::Rejected
                );
                let deinit_pkts = self.monitor.send_control(
                    &mut self.pica,
                    handle,
                    SessionDeinitCmd {
                        session_token: temp_id,
                    },
                    self.now,
                );
                assert_eq!(
                    UciMonitor::decode_response::<SessionDeinitRsp>(&deinit_pkts, handle).status,
                    Status::Ok
                );
            }
            _ => {
                let temp_id = 0x7000_0002;
                tc.assume(!self.monitor.has_session(handle, temp_id));
                self.probe_unconfigured_fira_session(handle, temp_id);
            }
        }
    }

    #[rule(weight = 2.0)]
    fn advance_clock(&mut self, tc: TestCase) {
        let jump_to_deadline = tc.draw(gs::booleans());
        if jump_to_deadline {
            if let Some(deadline) = self.pica.next_deadline() {
                self.now = self.now.max(deadline);
            }
        } else {
            let dt_ms = tc.draw(gs::integers::<u64>().min_value(0).max_value(120));
            self.now += Duration::from_millis(dt_ms);
        }
        self.pica.tick(self.now);
        let packets = self.monitor.observe_drain(&mut self.pica);
        for &(handle, ref bytes) in &packets {
            if let Some((token, is_dl_tdoa)) = UciMonitor::session_info_ntf(bytes)
                && let Some(kind) = self.sessions.get(&(handle, token))
            {
                match kind {
                    SessionKind::DlTdoaTag => {
                        assert!(
                            is_dl_tdoa,
                            "expected DlTdoa SessionInfoNtf for DT-Tag session {token} on handle {handle}"
                        );
                    }
                    SessionKind::Twr {
                        is_controller,
                        controlees,
                        controlee_len,
                        peer_handle,
                        peer_mac,
                        max_measurements,
                        ..
                    } => {
                        let summary = UciMonitor::decode_two_way_ntf(bytes)
                            .expect("Twr session should emit TwoWay SessionInfoNtf");
                        assert_eq!(summary.session_token, token);
                        assert!(summary.current_ranging_interval > 0);
                        let live_expected =
                            self.estimator.get(handle, *peer_handle).map(|m| m.range);
                        let allowed: Vec<MacAddress> = controlees
                            .get(..*controlee_len)
                            .unwrap_or(&[])
                            .iter()
                            .flatten()
                            .copied()
                            .collect();
                        for meas in &summary.measurements {
                            assert!(
                                allowed.contains(&meas.mac),
                                "SessionInfoNtf reported unknown peer {:?} not in {:?}",
                                meas.mac,
                                allowed
                            );
                            if meas.status == Status::Ok {
                                assert_eq!(Some(meas.distance), live_expected);
                            } else {
                                assert_eq!(meas.status, Status::RangingRxTimeout);
                                if live_expected.is_none() {
                                    assert_eq!(meas.distance, 0);
                                }
                            }
                        }
                        if *is_controller
                            && let Some(expected_range) = live_expected
                            && *max_measurements == 0
                            && allowed.contains(peer_mac)
                            && self
                                .monitor
                                .device(*peer_handle)
                                .and_then(|d| d.sessions.get(&token))
                                .is_some_and(|s| s.state == SessionState::SessionStateActive)
                            && matches!(
                                self.sessions.get(&(*peer_handle, token)),
                                Some(SessionKind::Twr {
                                    is_controller: false,
                                    peer_handle: back_peer,
                                    ..
                                }) if *back_peer == handle
                            )
                            && let Some(peer_meas) =
                                summary.measurements.iter().find(|m| m.mac == *peer_mac)
                        {
                            assert_eq!(peer_meas.status, Status::Ok);
                            assert_eq!(peer_meas.distance, expected_range);
                        }
                    }
                    SessionKind::CccOrAliro => {}
                }
            } else if let Some(rcv) = UciMonitor::decode_data_rcv(bytes) {
                let sender_match = self
                    .in_flight_data
                    .iter()
                    .find(|(key, pending)| {
                        key.1 == rcv.session_handle
                            && pending.peer_handle == handle
                            && pending.expect_delivery
                            && u64::from(pending.src_mac) == rcv.source_address
                    })
                    .map(|(&k, _)| k);
                if let Some(sender_key) = sender_match
                    && let Some(pending) = self.in_flight_data.get(&sender_key)
                {
                    assert_eq!(rcv.data_sequence_number, pending.seq_num);
                    assert_eq!(rcv.application_data, pending.payload);
                }
            } else if let Some(status_ntf) = UciMonitor::decode_data_status_ntf(bytes)
                && let Some(pending) = self
                    .in_flight_data
                    .remove(&(handle, status_ntf.session_token))
            {
                assert_eq!(status_ntf.uci_sequence_number, pending.seq_num);
                if pending.expect_delivery {
                    assert!(matches!(
                        status_ntf.status,
                        DataTransferNtfStatusCode::UciDataTransferStatusOk
                            | DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransfer
                    ));
                }
            }
        }
        self.sync_session_presence();
    }

    #[invariant]
    fn devices_anchors_and_deadlines_match(&mut self, _tc: TestCase) {
        let actual_devices: BTreeSet<Handle> = self.pica.devices().collect();
        assert_eq!(actual_devices, self.live_devices);
        let actual_anchors: BTreeMap<Handle, MacAddress> = self.pica.anchors().collect();
        assert_eq!(actual_anchors, self.active_anchors);
        assert_eq!(
            self.pica.next_deadline().is_some(),
            self.monitor.any_active()
        );
    }

    #[rule(weight = 0.8)]
    fn query_device_and_session_state(&mut self, tc: TestCase) {
        let handle = *tc.draw(self.devices.values_reusable());
        let _ = self
            .monitor
            .send_control(&mut self.pica, handle, SessionGetCountCmd {}, self.now);
        let cfg_pkts = self.monitor.send_control(
            &mut self.pica,
            handle,
            CoreGetConfigCmd {
                parameter_ids: vec![ConfigParameterId::DeviceState],
            },
            self.now,
        );
        let cfg_rsp: CoreGetConfigRsp = UciMonitor::decode_response(&cfg_pkts, handle);
        assert_eq!(cfg_rsp.status, Status::Ok);
        let session_ids: Vec<u32> = self
            .monitor
            .device(handle)
            .map(|d| d.sessions.keys().copied().collect())
            .unwrap_or_default();
        for session_id in session_ids {
            let state_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionGetStateCmd {
                    session_token: session_id,
                },
                self.now,
            );
            let state_rsp: SessionGetStateRsp = UciMonitor::decode_response(&state_pkts, handle);
            self.monitor
                .observe_session_query(handle, session_id, &state_rsp);

            let cnt_pkts = self.monitor.send_control(
                &mut self.pica,
                handle,
                SessionGetRangingCountCmd { session_id },
                self.now,
            );
            let cnt_rsp: SessionGetRangingCountRsp = UciMonitor::decode_response(&cnt_pkts, handle);
            self.monitor
                .observe_ranging_count(handle, session_id, &cnt_rsp);
        }
    }
}

#[hegel::test]
fn uci_host_state_machine(tc: TestCase) {
    let model = HostModel::new(&tc);
    machine(model).steps(30).run(tc);
}
