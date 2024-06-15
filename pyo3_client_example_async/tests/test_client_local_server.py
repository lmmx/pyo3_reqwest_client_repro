from asyncio import gather, Semaphore
import json
from pytest import mark

from pytest import mark

from pyo3_client_example_async import ReqwestClient


@mark.asyncio
@mark.parametrize("url", ["http://127.0.0.1:8000/hi"])
async def test_client_get_multi_test_server(url):
    """Install httpolars and run:

    uvicorn httpolars.test_utils.rate_limit_server:app --host 127.0.0.1 --port 8000
    """
    client = ReqwestClient()
    semaphore = Semaphore(1000)

    async def run_get(idx: int):
        url_i = f"{url}?number={idx}"
        async with semaphore:
            response_i = await client.get(url_i)
            return response_i

    num_tasks = 10000
    tasks = [run_get(i) for i in range(num_tasks)]
    results = await gather(*tasks)
    for idx, r in enumerate(results):
        parsed = json.loads(r)
        assert int(parsed.get("Hi")) == idx
