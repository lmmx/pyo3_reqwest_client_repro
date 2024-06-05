use pyo3::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[pyclass]
struct FileHandler {
    file: Option<File>,
}

#[pymethods]
impl FileHandler {
    #[new]
    fn new(path: &str, mode: &str) -> PyResult<Self> {
        let file = match mode {
            "r" => Some(File::open(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?),
            "w" => Some(File::create(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?),
            "a" => Some(OpenOptions::new().append(true).open(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?),
            _ => None,
        };
        Ok(FileHandler { file })
    }

    fn read(&mut self) -> PyResult<String> {
        let file = self.file.as_mut().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyIOError, _>("File not opened"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(contents)
    }

    fn write(&mut self, data: &str) -> PyResult<()> {
        let file = self.file.as_mut().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyIOError, _>("File not opened"))?;
        file.write_all(data.as_bytes())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(())
    }

    fn close(&mut self) {
        self.file = None;
    }
}

#[pyfunction]
fn append_text(handler: &Bound<FileHandler>, text: &str) -> PyResult<()> {
    let mut handler_ref = handler.borrow_mut();
    handler_ref.write(text)?;
    Ok(())
}

#[pymodule]
fn pyo3_file_handler_example(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FileHandler>()?;
    m.add_function(wrap_pyfunction!(append_text, m)?)?;
    Ok(())
}
