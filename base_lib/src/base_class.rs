#![allow(clippy::borrow_as_ptr)]

use pyo3::{prelude::*, types::PyInt};

#[pyclass(name = "Base", subclass)]
#[allow(clippy::module_name_repetitions)]
pub struct PyBase {
    #[pyo3(get)]
    base_attr: i64,
}

#[pymethods]
impl PyBase {
    /// # Errors
    #[new]
    pub fn new(base_attr: &PyInt) -> PyResult<Self> {
        Ok(Self {
            base_attr: base_attr.extract()?,
        })
    }

    /// # Errors
    pub fn base_add(&self, value: &PyInt) -> PyResult<i64> {
        Ok(self.base_attr + value.extract::<i64>()?)
    }
}
