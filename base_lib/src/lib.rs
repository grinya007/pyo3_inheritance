#![deny(rust_2018_idioms)]
#![deny(clippy::correctness)]
#![deny(clippy::perf)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]

pub mod base_class;

use pyo3::prelude::*;

use crate::base_class::PyBase;

/// The name of this function must match the `lib.name` setting in the `Cargo.toml`.
#[pymodule]
fn base_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBase>()?;

    Ok(())
}
