import pyo3_sleep_example
from pytest import mark

@mark.asyncio
async def test_sleep(capfd):
    await pyo3_sleep_example.rust_sleep()
    captured = capfd.readouterr()
    assert captured.out == "hello from rust!\n"
