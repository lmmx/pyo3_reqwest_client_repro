import pyo3_sleep_example

from asyncio import gather
from pytest import mark


@mark.asyncio
async def test_sleep(capfd):
    await pyo3_sleep_example.rust_sleep()
    captured = capfd.readouterr()
    assert captured.out == "hello from rust!\n"


@mark.asyncio
async def test_sleep_multi(capfd):
    async def run_sleep_and_capture():
        await pyo3_sleep_example.rust_sleep()

    num_tasks = 5000
    tasks = [run_sleep_and_capture() for _ in range(num_tasks)]
    results = await gather(*tasks)
    captured = capfd.readouterr()
    assert captured.out == "hello from rust!\n" * num_tasks
