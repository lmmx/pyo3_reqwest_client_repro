from pytest import mark

from pyo3_client_example import ReqwestClient


@mark.parametrize("url,expected", [("https://jsonplaceholder.typicode.com/todos/1", """\
{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}\
""")])
def test_client_get(url, expected):
    client = ReqwestClient()
    response = client.get(url)
    assert response == expected
