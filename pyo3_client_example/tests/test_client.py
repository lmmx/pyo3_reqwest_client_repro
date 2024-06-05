from pytest import mark

from pyo3_client_example import ReqwestClient


@mark.parametrize("url, expected", [("https://jsonplaceholder.typicode.com/todos/",
("""\
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
)
)])
def test_client_get(url, expected):
    url1 = url + "1"
    url2 = url + "2"
    expected1, expected2 = expected
    client = ReqwestClient()
    response1 = client.get(url1)
    assert response1 == expected1
    response2 = client.get(url2)
    assert response2 == expected2
