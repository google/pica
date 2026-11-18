import asyncio
from contextlib import asynccontextmanager
import pytest
import pytest_asyncio
import logging
import os

from datetime import datetime
from pathlib import Path
from typing import Tuple

from pica import Host
from pica.packets import uci

from .helper import init
from . import ranging, data_transfer

PICA_BIN = Path("./target/debug/pica")
DATA_FILE = Path("README.md")
PICA_LOCALHOST = "127.0.0.1"
CCC_SID = 20
MAX_MEASUREMENTS = 3

logging.basicConfig(level=os.environ.get("PICA_LOGLEVEL", "DEBUG").upper())


def setup_artifacts(test_name: str) -> Tuple[Path, Path]:
    artifacts = Path("./artifacts")
    artifacts.mkdir(parents=True, exist_ok=True)

    current_dt = datetime.now()
    formatted_date = current_dt.strftime("%Y-%m-%d_%H-%M-%S-%f")[:-3]

    f1 = artifacts / f"{formatted_date}_pica_{test_name}_stdout.txt"
    f1.touch(exist_ok=True)

    f2 = artifacts / f"{formatted_date}_pica_{test_name}_stderr.txt"
    f2.touch(exist_ok=True)

    return (f1, f2)


@pytest_asyncio.fixture
async def pica_port(request, unused_tcp_port):
    (stdout, stderr) = setup_artifacts(request.node.name)
    if not PICA_BIN.exists():
        raise FileNotFoundError(f"{PICA_BIN} not found")

    with stdout.open("w") as fstdout, stderr.open("w") as fstderr:
        process = await asyncio.create_subprocess_exec(
            PICA_BIN,
            "--uci-port",
            str(unused_tcp_port),
            stdout=fstdout,
            stderr=fstderr,
        )
        await asyncio.sleep(100 / 1000)  # Let pica boot up

        yield unused_tcp_port

        process.terminate()
        await process.wait()


@pytest.mark.asyncio
async def test_ranging(pica_port):
    await ranging.run(PICA_LOCALHOST, pica_port)


@pytest.mark.asyncio
async def test_data_transfer(pica_port):
    await data_transfer.run(PICA_LOCALHOST, pica_port, DATA_FILE)


@asynccontextmanager
async def connected_hosts(pica_port: int, count: int):
    hosts = []
    try:
        for i in range(count):
            host = await Host.connect(PICA_LOCALHOST, pica_port, bytes([0, i + 1]))
            await init(host)
            hosts.append(host)
        yield hosts
    finally:
        for host in hosts:
            host.disconnect()


def ccc_cfg(
    role: uci.DeviceRole,
    *,
    ntf_config: uci.SessionInfoNtfConfig = uci.SessionInfoNtfConfig.DISABLE,
    max_measurements: int = 0,
    extra: dict | None = None,
) -> dict:
    cfg = {
        uci.AppConfigTlvType.DEVICE_ROLE: [role],
        uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_MANY],
        uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [ntf_config],
        uci.AppConfigTlvType.MAX_NUMBER_OF_MEASUREMENTS: max_measurements.to_bytes(
            2, "little"
        ),
    }
    if extra:
        cfg.update(extra)
    return cfg


def assert_single_zero_mac_ok(
    rounds: list[list], expected_count: int = MAX_MEASUREMENTS
):
    assert len(rounds) == expected_count
    assert all(
        len(m) == 1 and m[0].status == uci.Status.OK and m[0].mac_address == 0
        for m in rounds
    )


async def run_positive_pairing(
    pica_port: int,
    session_type: uci.SessionType,
    extra_init: dict | None = None,
    extra_resp: dict | None = None,
):
    async with connected_hosts(pica_port, 2) as (host0, host1):
        await ranging.init_session(
            host0,
            CCC_SID,
            session_type,
            ccc_cfg(
                uci.DeviceRole.INITIATOR,
                ntf_config=uci.SessionInfoNtfConfig.ENABLE,
                max_measurements=MAX_MEASUREMENTS,
                extra=extra_init,
            ),
        )
        await ranging.init_session(
            host1,
            CCC_SID,
            session_type,
            ccc_cfg(uci.DeviceRole.RESPONDER, extra=extra_resp),
        )
        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host0, CCC_SID)
        rounds = await ranging.collect(host0, CCC_SID, MAX_MEASUREMENTS)
        assert_single_zero_mac_ok(rounds)
        await ranging.deinit_session(host0, CCC_SID)
        await ranging.stop_session(host1, CCC_SID)
        await ranging.deinit_session(host1, CCC_SID)


async def run_rejected_pairing(
    pica_port: int,
    st0: uci.SessionType,
    cfg0: dict,
    st1: uci.SessionType,
    cfg1: dict,
):
    async with connected_hosts(pica_port, 2) as (host0, host1):
        await ranging.init_session(host0, CCC_SID, st0, cfg0)
        await ranging.init_session(host1, CCC_SID, st1, cfg1)
        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host0, CCC_SID)
        for rounds in await asyncio.gather(
            ranging.collect(host0, CCC_SID, MAX_MEASUREMENTS),
            ranging.collect(host1, CCC_SID, MAX_MEASUREMENTS),
        ):
            assert rounds == [[] for _ in range(MAX_MEASUREMENTS)]
        await ranging.deinit_session(host0, CCC_SID)
        await ranging.deinit_session(host1, CCC_SID)


@pytest.mark.asyncio
@pytest.mark.parametrize("session_type", [uci.SessionType.CCC, uci.SessionType.ALIRO])
async def test_ccc_aliro_pairing(pica_port, session_type):
    await run_positive_pairing(pica_port, session_type)


@pytest.mark.asyncio
async def test_ccc_ignores_device_type(pica_port):
    await run_positive_pairing(
        pica_port,
        uci.SessionType.CCC,
        extra_init={uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CCC_CONTROLLER]},
        extra_resp={uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CCC_CONTROLLER]},
    )


@pytest.mark.asyncio
async def test_ccc_ignores_uwb_initiation_time(pica_port):
    await run_positive_pairing(
        pica_port,
        uci.SessionType.CCC,
        extra_init={
            uci.AppConfigTlvType.UWB_INITIATION_TIME: int(1000).to_bytes(8, "little")
        },
        extra_resp={
            uci.AppConfigTlvType.UWB_INITIATION_TIME: int(2000).to_bytes(8, "little")
        },
    )


@pytest.mark.asyncio
@pytest.mark.parametrize("session_type", [uci.SessionType.CCC, uci.SessionType.ALIRO])
async def test_ccc_aliro_rejects_both_initiators(pica_port, session_type):
    cfg = ccc_cfg(
        uci.DeviceRole.INITIATOR,
        ntf_config=uci.SessionInfoNtfConfig.ENABLE,
        max_measurements=MAX_MEASUREMENTS,
    )
    await run_rejected_pairing(pica_port, session_type, cfg, session_type, cfg)


@pytest.mark.asyncio
@pytest.mark.parametrize("session_type", [uci.SessionType.CCC, uci.SessionType.ALIRO])
async def test_ccc_aliro_rejects_preamble_mismatch(pica_port, session_type):
    await run_rejected_pairing(
        pica_port,
        session_type,
        ccc_cfg(
            uci.DeviceRole.INITIATOR,
            ntf_config=uci.SessionInfoNtfConfig.ENABLE,
            max_measurements=MAX_MEASUREMENTS,
        ),
        session_type,
        ccc_cfg(
            uci.DeviceRole.RESPONDER,
            ntf_config=uci.SessionInfoNtfConfig.ENABLE,
            max_measurements=MAX_MEASUREMENTS,
            extra={uci.AppConfigTlvType.PREAMBLE_CODE_INDEX: [11]},
        ),
    )


@pytest.mark.asyncio
async def test_ccc_vs_aliro_rejected(pica_port):
    await run_rejected_pairing(
        pica_port,
        uci.SessionType.CCC,
        ccc_cfg(
            uci.DeviceRole.INITIATOR,
            ntf_config=uci.SessionInfoNtfConfig.ENABLE,
            max_measurements=MAX_MEASUREMENTS,
        ),
        uci.SessionType.ALIRO,
        ccc_cfg(
            uci.DeviceRole.RESPONDER,
            ntf_config=uci.SessionInfoNtfConfig.ENABLE,
            max_measurements=MAX_MEASUREMENTS,
        ),
    )


@pytest.mark.asyncio
async def test_ccc_initiator_suspension_pauses_responder(pica_port):
    shared = {
        uci.AppConfigTlvType.RANGING_DURATION: int(200).to_bytes(4, "little"),
        uci.AppConfigTlvType.STS_INDEX: int(100).to_bytes(4, "little"),
    }
    async with connected_hosts(pica_port, 2) as (host0, host1):
        await ranging.init_session(
            host0,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(uci.DeviceRole.INITIATOR, extra=shared),
        )
        await ranging.init_session(
            host1,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(
                uci.DeviceRole.RESPONDER,
                ntf_config=uci.SessionInfoNtfConfig.ENABLE,
                extra=shared,
            ),
        )
        await ranging.start_session(host0, CCC_SID)
        await ranging.start_session(host1, CCC_SID)

        ntf = await host1.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
        assert_single_zero_mac_ok([ntf.two_way_ranging_measurements], 1)

        await ranging.set_app_config(
            host0,
            CCC_SID,
            {
                uci.AppConfigTlvType.SUSPEND_RANGING_ROUNDS: [
                    uci.SuspendRangingRounds.ENABLE
                ]
            },
        )
        host1.send_control(
            uci.SessionGetAppConfigCmd(session_token=CCC_SID, app_cfg=[0xA8])
        )
        while True:
            pkt = await host1.expect_control(uci.ControlPacket)
            if isinstance(pkt, uci.SessionGetAppConfigRsp):
                rsp_before = pkt
                break
            assert isinstance(pkt, uci.ShortMacTwoWaySessionInfoNtf)
        assert rsp_before.status == uci.Status.OK
        assert len(rsp_before.tlvs) == 1
        assert rsp_before.tlvs[0].cfg_id == 0xA8
        assert int.from_bytes(bytes(rsp_before.tlvs[0].v), "little") >= 100

        with pytest.raises(asyncio.TimeoutError):
            await host1.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=0.5)

        host1.send_control(
            uci.SessionGetAppConfigCmd(session_token=CCC_SID, app_cfg=[0xA8])
        )
        rsp_after = await host1.expect_control(uci.SessionGetAppConfigRsp)
        assert rsp_after == rsp_before

        await ranging.stop_session(host1, CCC_SID)
        await ranging.deinit_session(host1, CCC_SID)
        await ranging.stop_session(host0, CCC_SID)
        await ranging.deinit_session(host0, CCC_SID)


@pytest.mark.asyncio
async def test_ccc_suspended_responder_does_not_block_active_responder(pica_port):
    async with connected_hosts(pica_port, 3) as (host0, host1, host2):
        await ranging.init_session(
            host0,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(
                uci.DeviceRole.INITIATOR,
                ntf_config=uci.SessionInfoNtfConfig.ENABLE,
                max_measurements=MAX_MEASUREMENTS,
            ),
        )
        await ranging.init_session(
            host1,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(
                uci.DeviceRole.RESPONDER,
                extra={
                    uci.AppConfigTlvType.SUSPEND_RANGING_ROUNDS: [
                        uci.SuspendRangingRounds.ENABLE
                    ]
                },
            ),
        )
        await ranging.init_session(
            host2,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(uci.DeviceRole.RESPONDER),
        )

        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        await ranging.start_session(host0, CCC_SID)

        rounds = await ranging.collect(host0, CCC_SID, MAX_MEASUREMENTS)
        assert_single_zero_mac_ok(rounds)

        await ranging.deinit_session(host0, CCC_SID)
        await ranging.stop_session(host1, CCC_SID)
        await ranging.deinit_session(host1, CCC_SID)
        await ranging.stop_session(host2, CCC_SID)
        await ranging.deinit_session(host2, CCC_SID)


def slot_measurements(measurements: list) -> list[tuple[int, uci.Status, int]]:
    return [(m.slot_index, m.status, m.mac_address) for m in measurements]


async def set_ntf(
    host: Host,
    ntf_config: uci.SessionInfoNtfConfig,
    max_measurements: int = 0,
):
    await ranging.set_app_config(
        host,
        CCC_SID,
        {
            uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [ntf_config],
            uci.AppConfigTlvType.MAX_NUMBER_OF_MEASUREMENTS: (
                max_measurements.to_bytes(2, "little")
            ),
        },
    )


async def init_multi_anchor(
    initiator: Host,
    responders: list[tuple[Host, int | None]],
    *,
    session_type: uci.SessionType = uci.SessionType.CCC,
    ntf_config: uci.SessionInfoNtfConfig = uci.SessionInfoNtfConfig.ENABLE,
    max_measurements: int = 0,
    num_controlees: int | None = None,
):
    await ranging.init_session(
        initiator,
        CCC_SID,
        session_type,
        ccc_cfg(
            uci.DeviceRole.INITIATOR,
            ntf_config=ntf_config,
            max_measurements=max_measurements,
            extra={
                uci.AppConfigTlvType.NUMBER_OF_CONTROLEES: [
                    num_controlees if num_controlees is not None else len(responders)
                ]
            },
        ),
    )
    for responder, slot in responders:
        extra = (
            {uci.AppConfigTlvType.RESPONDER_SLOT_INDEX: [slot]}
            if slot is not None
            else None
        )
        await ranging.init_session(
            responder,
            CCC_SID,
            session_type,
            ccc_cfg(uci.DeviceRole.RESPONDER, extra=extra),
        )


async def stop_session_draining_ntfs(host: Host, sid: int):
    host.send_control(uci.SessionStopCmd(session_id=sid))
    while True:
        pkt = await host.expect_control(uci.ControlPacket)
        if isinstance(pkt, uci.SessionStopRsp):
            assert pkt.status == uci.Status.OK
            break
        assert isinstance(pkt, uci.ShortMacTwoWaySessionInfoNtf)
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=sid,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )
    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )


async def get_ranging_count(host: Host, sid: int) -> int:
    host.send_control(uci.SessionGetRangingCountCmd(session_id=sid))
    rsp = await host.expect_control(uci.SessionGetRangingCountRsp)
    assert rsp.status == uci.Status.OK
    return rsp.count


async def wait_for_ranging_round(host: Host, sid: int):
    count_before = await get_ranging_count(host, sid)
    deadline = asyncio.get_running_loop().time() + 2.0
    while await get_ranging_count(host, sid) <= count_before:
        assert asyncio.get_running_loop().time() < deadline
        await asyncio.sleep(0.05)


def fira_responder_cfg(
    host: Host,
    *,
    slots_per_rr: int = 1,
    slot_index: int | None = None,
) -> dict:
    cfg = {
        uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.RESPONDER],
        uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_ONE],
        uci.AppConfigTlvType.RANGING_ROUND_USAGE: [
            uci.RangingRoundUsage.DS_TWR_DEFERRED_MODE
        ],
        uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host.mac_address,
        uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
        uci.AppConfigTlvType.SLOTS_PER_RR: [slots_per_rr],
        uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
            uci.SessionInfoNtfConfig.DISABLE
        ],
    }
    if slot_index is not None:
        cfg[uci.AppConfigTlvType.RESPONDER_SLOT_INDEX] = [slot_index]
    return cfg


@pytest.mark.asyncio
async def test_responder_slot_index_zero_rejected(pica_port):
    async with connected_hosts(pica_port, 1) as (host,):
        await ranging.init_session(
            host,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(uci.DeviceRole.RESPONDER),
        )
        host.send_control(
            uci.SessionSetAppConfigCmd(
                session_token=CCC_SID,
                tlvs=[
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.RESPONDER_SLOT_INDEX,
                        v=bytes([0]),
                    )
                ],
            )
        )
        rsp = await host.expect_control(uci.SessionSetAppConfigRsp)
        assert rsp.status == uci.Status.INVALID_PARAM
        assert {s.cfg_id: s.status for s in rsp.cfg_status} == {
            uci.AppConfigTlvType.RESPONDER_SLOT_INDEX: uci.Status.INVALID_PARAM
        }
        await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_responder_slot_index_ge_slots_per_rr_rejected(pica_port):
    async with connected_hosts(pica_port, 1) as (host,):
        await ranging.init_session(
            host,
            CCC_SID,
            uci.SessionType.CCC,
            ccc_cfg(
                uci.DeviceRole.RESPONDER,
                extra={
                    uci.AppConfigTlvType.SLOTS_PER_RR: [6],
                    uci.AppConfigTlvType.RESPONDER_SLOT_INDEX: [6],
                },
            ),
        )
        await ranging.expect_session_start_error(
            host, CCC_SID, uci.ReasonCode.ERROR_INSUFFICIENT_SLOTS_PER_RR
        )
        await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_fira_responder_slots_per_rr_1_without_slot_index_starts(pica_port):
    async with connected_hosts(pica_port, 1) as (host,):
        await ranging.init_session(
            host,
            CCC_SID,
            uci.SessionType.FIRA_RANGING_SESSION,
            fira_responder_cfg(host, slots_per_rr=1),
        )
        await ranging.start_session(host, CCC_SID)
        await ranging.stop_session(host, CCC_SID)
        await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_fira_responder_explicit_slot_index_ge_slots_per_rr_rejected(pica_port):
    async with connected_hosts(pica_port, 1) as (host,):
        await ranging.init_session(
            host,
            CCC_SID,
            uci.SessionType.FIRA_RANGING_SESSION,
            fira_responder_cfg(host, slots_per_rr=1, slot_index=1),
        )
        await ranging.expect_session_start_error(
            host, CCC_SID, uci.ReasonCode.ERROR_INSUFFICIENT_SLOTS_PER_RR
        )
        await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
@pytest.mark.parametrize("session_type", [uci.SessionType.CCC, uci.SessionType.ALIRO])
async def test_ccc_aliro_multi_anchor(pica_port, session_type):
    async with connected_hosts(pica_port, 3) as (host0, host1, host2):
        await init_multi_anchor(
            host0,
            [(host1, 1), (host2, 2)],
            session_type=session_type,
            max_measurements=MAX_MEASUREMENTS,
        )

        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        await ranging.start_session(host0, CCC_SID)
        init_rounds = await ranging.collect(host0, CCC_SID, MAX_MEASUREMENTS)
        assert [slot_measurements(r) for r in init_rounds] == [
            [(1, uci.Status.OK, 0), (2, uci.Status.OK, 0)]
            for _ in range(MAX_MEASUREMENTS)
        ]

        await ranging.stop_session(host1, CCC_SID)
        await ranging.stop_session(host2, CCC_SID)

        await set_ntf(host0, uci.SessionInfoNtfConfig.DISABLE)
        for responder in (host1, host2):
            await set_ntf(responder, uci.SessionInfoNtfConfig.ENABLE, MAX_MEASUREMENTS)

        await ranging.start_session(host0, CCC_SID)
        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        resp_rounds = await asyncio.gather(
            ranging.collect(host1, CCC_SID, MAX_MEASUREMENTS),
            ranging.collect(host2, CCC_SID, MAX_MEASUREMENTS),
        )
        for rounds in resp_rounds:
            assert [slot_measurements(r) for r in rounds] == [
                [(0, uci.Status.OK, 0)] for _ in range(MAX_MEASUREMENTS)
            ]

        await ranging.stop_session(host0, CCC_SID)
        for host in (host0, host1, host2):
            await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_ccc_multi_anchor_over_budget_slot_excluded(pica_port):
    async with connected_hosts(pica_port, 3) as (host0, host1, host2):
        await init_multi_anchor(
            host0,
            [(host1, 1), (host2, 2)],
            max_measurements=MAX_MEASUREMENTS,
            num_controlees=1,
        )

        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        await ranging.start_session(host0, CCC_SID)
        init_rounds = await ranging.collect(host0, CCC_SID, MAX_MEASUREMENTS)
        assert [slot_measurements(r) for r in init_rounds] == [
            [(1, uci.Status.OK, 0)] for _ in range(MAX_MEASUREMENTS)
        ]

        await ranging.stop_session(host2, CCC_SID)
        await set_ntf(host0, uci.SessionInfoNtfConfig.DISABLE)
        await set_ntf(host2, uci.SessionInfoNtfConfig.ENABLE, MAX_MEASUREMENTS)
        await ranging.start_session(host0, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        host2_rounds = await ranging.collect(host2, CCC_SID, MAX_MEASUREMENTS)
        assert host2_rounds == [[] for _ in range(MAX_MEASUREMENTS)]

        await ranging.stop_session(host0, CCC_SID)
        await ranging.stop_session(host1, CCC_SID)
        for host in (host0, host1, host2):
            await ranging.deinit_session(host, CCC_SID)


async def run_slot_collision(pica_port: int, slot: int | None):
    async with connected_hosts(pica_port, 3) as hosts:
        host0, host1, host2 = hosts
        await init_multi_anchor(
            host0,
            [(host1, slot), (host2, slot)],
            ntf_config=uci.SessionInfoNtfConfig.DISABLE,
        )

        for measured in hosts:
            others = [h for h in hosts if h is not measured]
            for other in others:
                await set_ntf(other, uci.SessionInfoNtfConfig.DISABLE)
                await ranging.start_session(other, CCC_SID)
            await set_ntf(measured, uci.SessionInfoNtfConfig.ENABLE, MAX_MEASUREMENTS)
            await ranging.start_session(measured, CCC_SID)
            rounds = await ranging.collect(measured, CCC_SID, MAX_MEASUREMENTS)
            assert rounds == [[] for _ in range(MAX_MEASUREMENTS)]
            for other in others:
                await ranging.stop_session(other, CCC_SID)

        for host in hosts:
            await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_ccc_explicit_slot_collision(pica_port):
    await run_slot_collision(pica_port, 1)


@pytest.mark.asyncio
async def test_ccc_default_slot_collision(pica_port):
    await run_slot_collision(pica_port, None)


@pytest.mark.asyncio
async def test_ccc_slot_collision_does_not_spill(pica_port):
    async with connected_hosts(pica_port, 4) as (host0, host1, host2, host3):
        await init_multi_anchor(
            host0,
            [(host1, 1), (host2, 1), (host3, 3)],
            max_measurements=MAX_MEASUREMENTS,
            num_controlees=2,
        )
        for responder in (host1, host2, host3):
            await ranging.start_session(responder, CCC_SID)

        await ranging.start_session(host0, CCC_SID)
        rounds = await ranging.collect(host0, CCC_SID, MAX_MEASUREMENTS)
        assert rounds == [[] for _ in range(MAX_MEASUREMENTS)]

        for responder in (host1, host2, host3):
            await ranging.stop_session(responder, CCC_SID)
        for host in (host0, host1, host2, host3):
            await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_ccc_suspending_one_responder_keeps_other_ranging(pica_port):
    async with connected_hosts(pica_port, 3) as (host0, host1, host2):
        await init_multi_anchor(host0, [(host1, 1), (host2, 2)])

        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        await ranging.start_session(host0, CCC_SID)

        ntf = await host0.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
        assert slot_measurements(ntf.two_way_ranging_measurements) == [
            (1, uci.Status.OK, 0),
            (2, uci.Status.OK, 0),
        ]

        await ranging.set_app_config(
            host1,
            CCC_SID,
            {
                uci.AppConfigTlvType.SUSPEND_RANGING_ROUNDS: [
                    uci.SuspendRangingRounds.ENABLE
                ]
            },
        )
        for _ in range(5):
            ntf = await host0.expect_control(
                uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
            )
            if slot_measurements(ntf.two_way_ranging_measurements) == [
                (2, uci.Status.OK, 0)
            ]:
                break
        else:
            pytest.fail("host0 did not transition to ranging with host2 only")

        for _ in range(2):
            ntf = await host0.expect_control(
                uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
            )
            assert slot_measurements(ntf.two_way_ranging_measurements) == [
                (2, uci.Status.OK, 0)
            ]

        await stop_session_draining_ntfs(host0, CCC_SID)
        await ranging.deinit_session(host0, CCC_SID)
        for host in (host1, host2):
            await ranging.stop_session(host, CCC_SID)
            await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_ccc_multi_anchor_proximity_edge_trigger(pica_port):
    async with connected_hosts(pica_port, 3) as (host0, host1, host2):
        await init_multi_anchor(
            host0,
            [(host1, 1), (host2, 2)],
            ntf_config=uci.SessionInfoNtfConfig.ENABLE_PROXIMITY_EDGE_TRIGGER,
        )

        await ranging.start_session(host1, CCC_SID)
        await ranging.start_session(host2, CCC_SID)
        await ranging.start_session(host0, CCC_SID)

        entry_ntf = await host0.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
        )
        assert slot_measurements(entry_ntf.two_way_ranging_measurements) == [
            (1, uci.Status.OK, 0),
            (2, uci.Status.OK, 0),
        ]

        with pytest.raises(asyncio.TimeoutError):
            await host0.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=0.25)

        await ranging.stop_session(host1, CCC_SID)

        await wait_for_ranging_round(host0, CCC_SID)

        await ranging.start_session(host1, CCC_SID)
        reentry_ntf = await host0.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
        )
        assert slot_measurements(reentry_ntf.two_way_ranging_measurements) == [
            (1, uci.Status.OK, 0)
        ]

        await stop_session_draining_ntfs(host0, CCC_SID)
        await ranging.deinit_session(host0, CCC_SID)
        for host in (host1, host2):
            await ranging.stop_session(host, CCC_SID)
            await ranging.deinit_session(host, CCC_SID)


@pytest.mark.asyncio
async def test_fira_multicast_remove_readd_retriggers_edge(pica_port: int):
    sid = 1
    async with connected_hosts(pica_port, 3) as (host0, host1, host2):
        await ranging.init_session(
            host0,
            sid,
            uci.SessionType.FIRA_RANGING_SESSION,
            {
                uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.INITIATOR],
                uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CONTROLLER],
                uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host0.mac_address,
                uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
                uci.AppConfigTlvType.CHANNEL_NUMBER: [9],
                uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_MANY],
                uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
                uci.AppConfigTlvType.RANGING_ROUND_USAGE: [
                    uci.RangingRoundUsage.DS_TWR_DEFERRED_MODE
                ],
                uci.AppConfigTlvType.NUMBER_OF_CONTROLEES: [2],
                uci.AppConfigTlvType.DST_MAC_ADDRESS: (
                    host1.mac_address + host2.mac_address
                ),
                uci.AppConfigTlvType.IN_BAND_TERMINATION_ATTEMPT_COUNT: [0],
                uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
                    uci.SessionInfoNtfConfig.ENABLE_PROXIMITY_EDGE_TRIGGER
                ],
                uci.AppConfigTlvType.NEAR_PROXIMITY_CONFIG: int(0).to_bytes(
                    2, "little"
                ),
                uci.AppConfigTlvType.FAR_PROXIMITY_CONFIG: int(1000).to_bytes(
                    2, "little"
                ),
                uci.AppConfigTlvType.RANGING_DURATION: int(96).to_bytes(4, "little"),
            },
        )
        for host in (host1, host2):
            await ranging.init_session(
                host,
                sid,
                uci.SessionType.FIRA_RANGING_SESSION,
                {
                    uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.RESPONDER],
                    uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CONTROLEE],
                    uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host.mac_address,
                    uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
                    uci.AppConfigTlvType.CHANNEL_NUMBER: [9],
                    uci.AppConfigTlvType.MULTI_NODE_MODE: [
                        uci.MultiNodeMode.ONE_TO_MANY
                    ],
                    uci.AppConfigTlvType.SCHEDULE_MODE: [
                        uci.ScheduleMode.CONTENTION_BASED
                    ],
                    uci.AppConfigTlvType.RANGING_ROUND_USAGE: [
                        uci.RangingRoundUsage.DS_TWR_DEFERRED_MODE
                    ],
                    uci.AppConfigTlvType.NUMBER_OF_CONTROLEES: [1],
                    uci.AppConfigTlvType.DST_MAC_ADDRESS: host0.mac_address,
                    uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
                        uci.SessionInfoNtfConfig.DISABLE
                    ],
                },
            )

        await ranging.start_session(host1, sid)
        await ranging.start_session(host2, sid)
        await ranging.start_session(host0, sid)

        entry_ntf = await host0.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
        )
        assert {
            (m.mac_address, m.status) for m in entry_ntf.two_way_ranging_measurements
        } == {
            (int.from_bytes(host1.mac_address, "little"), uci.Status.OK),
            (int.from_bytes(host2.mac_address, "little"), uci.Status.OK),
        }

        controlee = [
            uci.Controlee(short_address=list(host1.mac_address), subsession_id=0)
        ]
        expected_status = [
            uci.ControleeStatus(
                mac_address=list(host1.mac_address),
                status=uci.MulticastUpdateStatus.OK_MULTICAST_LIST_UPDATE,
            )
        ]

        async def update_multicast_list(action: uci.UpdateMulticastListAction):
            host0.send_control(
                uci.SessionUpdateControllerMulticastListCmd(
                    session_token=sid,
                    action=action,
                    payload=uci.SessionUpdateControllerMulticastListCmdPayload(
                        controlees=controlee
                    ).serialize(),
                )
            )
            await host0.expect_control(
                uci.SessionUpdateControllerMulticastListRsp(
                    status=uci.Status.OK,
                    controlee_status=expected_status,
                )
            )

        await update_multicast_list(uci.UpdateMulticastListAction.REMOVE_CONTROLEE)
        await host0.expect_control(
            uci.SessionUpdateControllerMulticastListNtf(
                session_token=sid,
                controlee_status=expected_status,
            )
        )

        await wait_for_ranging_round(host0, sid)

        await update_multicast_list(uci.UpdateMulticastListAction.ADD_CONTROLEE)

        reentry_ntf = await host0.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
        )
        assert [
            (m.mac_address, m.status) for m in reentry_ntf.two_way_ranging_measurements
        ] == [(int.from_bytes(host1.mac_address, "little"), uci.Status.OK)]

        await stop_session_draining_ntfs(host0, sid)
        await ranging.deinit_session(host0, sid)
        for host in (host1, host2):
            await ranging.stop_session(host, sid)
            await ranging.deinit_session(host, sid)
