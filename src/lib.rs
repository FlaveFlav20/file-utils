use pyo3::prelude::*;

pub mod read_delim;

pub mod tail;
use tail::{tail_string, tail_, tail_string_delim};

////
#[pymodule]
fn file_operations_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tail_string, m)?)?;
    m.add_function(wrap_pyfunction!(tail_, m)?)?;
    m.add_function(wrap_pyfunction!(tail_string_delim, m)?)?;
    Ok(())
}
