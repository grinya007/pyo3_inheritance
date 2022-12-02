#![allow(clippy::borrow_as_ptr)]

use pyo3::{prelude::*, types::PyInt};

use base_lib::base_class::PyBase;

#[pyclass(name = "Sub1", extends = PyBase, subclass)]
#[allow(clippy::module_name_repetitions)]
pub struct PySub1 {
    #[pyo3(get)]
    sub1_attr: i64,
}

#[pymethods]
impl PySub1 {
    /// # Errors
    #[new]
    pub fn new(base_attr: &PyInt, sub1_attr: &PyInt) -> PyResult<(Self, PyBase)> {
        Ok((
            Self {
                sub1_attr: sub1_attr.extract()?,
            },
            PyBase::new(base_attr)?,
        ))
    }
}
