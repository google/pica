import asyncio
import pytest
import pytest_asyncio
import logging
import os

from datetime import datetime
from pathlib import Path
from typing import Tuple

from . import ranging, data_transfer

PICA_BIN = Path("./target/debug/pica")
DATA_FILE = Path("README.md")
PICA_LOCALHOST = "127.0.0.1"

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
