use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use tokio::time::sleep;

#[pyclass]
struct Counter {
    value: i32,
}

#[pymethods]
impl Counter {
    #[new]
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

#[pyfunction]
fn increment_counter(counter: &Bound<Counter>) {
    let mut counter = counter.borrow_mut();
    counter.increment();
}

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
    m.add_class::<Counter>()?;
    m.add_function(wrap_pyfunction!(increment_counter, m)?)?;
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    Ok(())
}
