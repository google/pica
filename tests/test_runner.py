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
