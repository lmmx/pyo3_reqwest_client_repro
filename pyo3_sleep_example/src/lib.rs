use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use tokio::time::sleep;

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async {
        sleep(std::time::Duration::from_secs(1)).await;
        println!("hello from rust!");
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pymodule]
fn pyo3_sleep_example(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    Ok(())
}
