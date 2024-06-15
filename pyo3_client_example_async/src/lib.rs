use pyo3::prelude::*;
use reqwest::Client;
use tokio::runtime::Runtime;
use pyo3::exceptions::PyIOError;

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

    fn get(&self, url: &str) -> PyResult<String> {
        let client = self.client.clone();
        let url = url.to_string();
        
        let result = self.runtime.block_on(async move {
            let response = client.get(&url).send().await.map_err(|e| PyIOError::new_err(e.to_string()))?;
            let text = response.text().await.map_err(|e| PyIOError::new_err(e.to_string()))?;
            Ok(text)
        });

        result
    }
}

#[pymodule]
fn pyo3_client_example_async(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ReqwestClient>()?;
    Ok(())
}
