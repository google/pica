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
    self, AoaResultReq, AppConfigTlv, AppConfigTlvType, DeviceRole, DeviceType,
    ExtendedMacTwoWaySessionInfoNtf, MacAddressMode, RssiReporting, SessionType,
    ShortMacTwoWaySessionInfoNtf,
};
use pica::{Handle, MacAddress, RangingMeasurement};

use crate::common::PicaHarness;
use crate::config::{PairwiseEstimator, twr_mac_pair, twr_session_tlvs};

fn expected_aoa_fields(measurement: RangingMeasurement, req: AoaResultReq) -> (u16, u8, u16, u8) {
    let az = (measurement.azimuth.saturating_mul(128)) as u16;
    let el = (i16::from(measurement.elevation).saturating_mul(128)) as u16;
    match req {
        AoaResultReq::AoaDisabled => (0, 0, 0, 0),
        AoaResultReq::AoaEnabled => (az, 100, el, 100),
        AoaResultReq::AoaEnabledAzimuthOnly => (az, 100, 0, 0),
        AoaResultReq::AoaEnabledElevationOnly => (0, 0, el, 100),
    }
}

fn assert_device_twr_report(
    harness: &mut PicaHarness,
    handle: Handle,
    mac_mode: MacAddressMode,
    expected_peer_mac: MacAddress,
    local: RangingMeasurement,
    aoa_req: AoaResultReq,
    rssi_reporting: RssiReporting,
) {
    let (exp_az, exp_az_fom, exp_el, exp_el_fom) = expected_aoa_fields(local, aoa_req);
    let exp_rssi = match rssi_reporting {
        RssiReporting::Enable => 100,
        RssiReporting::Disable => 0,
    };

    if mac_mode == MacAddressMode::Mode0 {
        let ntf = harness.tick_until::<ShortMacTwoWaySessionInfoNtf>(handle);
        let meas = ntf
            .two_way_ranging_measurements
            .first()
            .expect("expected short TWR measurement");
        assert_eq!(meas.status, uci::Status::Ok);
        assert_eq!(
            MacAddress::Short(meas.mac_address.to_le_bytes()),
            expected_peer_mac
        );
        assert_eq!(meas.distance, local.range);
        assert_eq!(meas.aoa_azimuth, exp_az);
        assert_eq!(meas.aoa_azimuth_fom, exp_az_fom);
        assert_eq!(meas.aoa_elevation, exp_el);
        assert_eq!(meas.aoa_elevation_fom, exp_el_fom);
        assert_eq!(meas.rssi, exp_rssi);
    } else {
        assert_eq!(mac_mode, MacAddressMode::Mode2);
        let ntf = harness.tick_until::<ExtendedMacTwoWaySessionInfoNtf>(handle);
        let meas = ntf
            .two_way_ranging_measurements
            .first()
            .expect("expected extended TWR measurement");
        assert_eq!(meas.status, uci::Status::Ok);
        assert_eq!(
            MacAddress::Extended(meas.mac_address.to_le_bytes()),
            expected_peer_mac
        );
        assert_eq!(meas.distance, local.range);
        assert_eq!(meas.aoa_azimuth, exp_az);
        assert_eq!(meas.aoa_azimuth_fom, exp_az_fom);
        assert_eq!(meas.aoa_elevation, exp_el);
        assert_eq!(meas.aoa_elevation_fom, exp_el_fom);
        assert_eq!(meas.rssi, exp_rssi);
    }
}

#[hegel::test]
fn twr_measurement_reports_estimator_distance(tc: TestCase) {
    let session_id = tc.draw(gs::integers::<u32>().min_value(1));
    let mac_mode = tc.draw(gs::sampled_from(vec![
        MacAddressMode::Mode0,
        MacAddressMode::Mode2,
    ]));
    let aoa_req = tc.draw(gs::sampled_from(vec![
        AoaResultReq::AoaDisabled,
        AoaResultReq::AoaEnabled,
        AoaResultReq::AoaEnabledAzimuthOnly,
        AoaResultReq::AoaEnabledElevationOnly,
    ]));
    let rssi_reporting = tc.draw(gs::sampled_from(vec![
        RssiReporting::Disable,
        RssiReporting::Enable,
    ]));
    let ctrl_meas = RangingMeasurement {
        range: tc.draw(gs::integers::<u16>().min_value(1)),
        azimuth: tc.draw(gs::integers::<i16>().min_value(-180).max_value(180)),
        elevation: tc.draw(gs::integers::<i8>().min_value(-90).max_value(90)),
    };
    let clee_meas = RangingMeasurement {
        range: tc.draw(gs::integers::<u16>().min_value(1)),
        azimuth: tc.draw(gs::integers::<i16>().min_value(-180).max_value(180)),
        elevation: tc.draw(gs::integers::<i8>().min_value(-90).max_value(90)),
    };
    let c0 = tc.draw(gs::integers::<u8>().min_value(1).max_value(127));
    let c1 = tc.draw(gs::integers::<u8>().min_value(1).max_value(127));
    let (controller_mac, controlee_mac) = twr_mac_pair(mac_mode, c0, c1);

    let estimator = PairwiseEstimator::default();
    let mut harness = PicaHarness::new(Box::new(estimator.clone()));
    let controller = harness.add_reset_device();
    let controlee = harness.add_reset_device();
    estimator.insert(controller, controlee, ctrl_meas);
    estimator.insert(controlee, controller, clee_meas);

    let extra = vec![
        AppConfigTlv {
            cfg_id: AppConfigTlvType::AoaResultReq,
            v: vec![aoa_req as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RssiReporting,
            v: vec![rssi_reporting as u8],
        },
    ];
    harness.init_and_configure(
        controller,
        session_id,
        SessionType::FiraRangingSession,
        twr_session_tlvs(
            DeviceRole::Initiator,
            DeviceType::Controller,
            mac_mode,
            controller_mac,
            controlee_mac,
            55,
            extra.clone(),
        ),
    );
    harness.init_and_configure(
        controlee,
        session_id,
        SessionType::FiraRangingSession,
        twr_session_tlvs(
            DeviceRole::Responder,
            DeviceType::Controlee,
            mac_mode,
            controlee_mac,
            controller_mac,
            55,
            extra,
        ),
    );

    harness.start_session(controlee, session_id);
    harness.start_session(controller, session_id);

    assert_device_twr_report(
        &mut harness,
        controller,
        mac_mode,
        controlee_mac,
        ctrl_meas,
        aoa_req,
        rssi_reporting,
    );
    assert_device_twr_report(
        &mut harness,
        controlee,
        mac_mode,
        controller_mac,
        clee_meas,
        aoa_req,
        rssi_reporting,
    );

    harness.stop_and_deinit_session(controller, session_id);
    harness.stop_and_deinit_session(controlee, session_id);
}
