from asyncio import gather
import json
from pytest import mark

from pytest import mark
from random import randint

from pyo3_client_example_async import ReqwestClient


@mark.parametrize(
    "url, expected",
    [
        (
            "https://jsonplaceholder.typicode.com/todos/",
            (
                """\
{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}\
""",
                """\
{
  "userId": 1,
  "id": 2,
  "title": "quis ut nam facilis et officia qui",
  "completed": false
}\
""",
            ),
        )
    ],
)
@mark.asyncio
async def test_client_get(url, expected):
    url1 = url + "1"
    url2 = url + "2"
    expected1, expected2 = expected
    client = ReqwestClient()
    response1 = await client.get(url1)
    assert response1 == expected1
    response2 = await client.get(url2)
    assert response2 == expected2


@mark.asyncio
@mark.parametrize("url", ["https://jsonplaceholder.typicode.com/todos/"])
async def test_client_get_multi(url):
    client = ReqwestClient()

    async def run_get():
        idx = randint(1, 100)
        url_i = f"{url}{idx}"
        response_i = await client.get(url_i)
        return response_i

    num_tasks = 1000
    tasks = [run_get() for _ in range(num_tasks)]
    results = await gather(*tasks)
    for r in results:
        parsed = json.loads(r)
        assert parsed.get("id") in range(1, 101)

@mark.asyncio
@mark.parametrize("url", ["http://127.0.0.1:8000/"])
async def test_client_get_multi_test_server(url):
    client = ReqwestClient()

    async def run_get(idx: int):
        url_i = f"{url}?value={idx}"
        response_i = await client.get(url_i)
        return response_i

    num_tasks = 1000
    tasks = [run_get(i) for i in range(num_tasks)]
    results = await gather(*tasks)
    for r in results:
        parsed = json.loads(r)
        assert parsed.get("Hello") == "World"
