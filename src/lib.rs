use pyo3::prelude::*;

pub mod utils;

pub mod with_eol;
use with_eol::WithEOL;

pub mod with_custom_delims;

////
#[pymodule]
fn file_utils_operations_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WithEOL>()?;
    Ok(())
}
