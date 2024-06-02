use pyo3::prelude::*;

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

#[pymodule]
fn pyo3_counter_example(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_function(wrap_pyfunction!(increment_counter, m)?)?;
    Ok(())
}
