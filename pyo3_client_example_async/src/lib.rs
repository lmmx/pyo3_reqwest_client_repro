use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use reqwest::Client;
use tokio::runtime::Runtime;
use tokio::task;

#[pyclass]
struct ReqwestClient {
    client: Client,
    runtime: Runtime,
}

#[pymethods]
impl ReqwestClient {
    #[new]
    fn new() -> PyResult<Self> {
        let client = Client::new();
        let runtime = Runtime::new().map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;
        Ok(ReqwestClient { client, runtime })
    }

    fn get<'a>(&'a self, py: Python<'a>, url: &'a str) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        let url = url.to_string();

        let fut = async move {
            let response = client.get(&url).send().await.map_err(|e| PyIOError::new_err(e.to_string()))?;
            let text = response.text().await.map_err(|e| PyIOError::new_err(e.to_string()))?;
            Ok(text)
        };

        let py_future = future_into_py(py, async move {
            match task::spawn(fut).await {
                Ok(result) => result,
                Err(err) => Err(PyIOError::new_err(err.to_string())),
            }
        })?;

        Ok(py_future)
    }
}

#[pymodule]
fn pyo3_client_example_async(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ReqwestClient>()?;
    Ok(())
}
