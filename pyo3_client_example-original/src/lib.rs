use pyo3::prelude::*;
use reqwest::Client;
use serde::Deserialize;

#[pyclass]
pub struct ApiClient {
    pub client: Client,
}

#[pymethods]
impl ApiClient {
    #[new]
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
        }
    }
}

#[derive(Deserialize)]
struct ApiCallKwargs {
    endpoint: String,
    // Use an optional client reference, which will be passed at runtime
    #[serde(skip_deserializing)]
    client: Option<Py<PyCell<ApiClient>>>,
}

#[pyfunction]
fn make_request(
    kwargs: ApiCallKwargs,
) -> PyResult<(String, u16)> {
    let default_client;
    let client = match kwargs.client {
        Some(ref client_cell) => {
            let api_client: &ApiClient = &client_cell.borrow();
            &api_client.client
        },
        None => {
            default_client = Client::new();
            &default_client
        },
    };

    let response = client
        .get(&kwargs.endpoint)
        .send()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    let status = response.status().as_u16();
    let text = response
        .text()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    Ok((text, status))
}

#[pymodule]
fn minimal_pyo3_extension(m: &PyModule) -> PyResult<()> {
    m.add_class::<ApiClient>()?;
    m.add_function(wrap_pyfunction!(make_request, m)?)?;
    Ok(())
}
