use pyo3::prelude::*;
use reqwest::blocking::Client;

#[pyclass]
struct ReqwestClient {
    client: Client,
}

#[pymethods]
impl ReqwestClient {
    #[new]
    fn new() -> PyResult<Self> {
        let client = Client::new();
        Ok(ReqwestClient { client })
    }

    fn get(&self, url: &str) -> PyResult<String> {
        let response = self.client
            .get(url)
            .send()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        let text = response
            .text()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(text)
    }

    // /// This would require making self.client optional etc
    // fn close(&mut self) {
    //    self.client = None;
    // }
}

#[pymodule]
fn pyo3_client_example_native_tls(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ReqwestClient>()?;
    Ok(())
}
