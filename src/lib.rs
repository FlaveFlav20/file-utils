use pyo3::prelude::*;

pub mod read_delim;

pub mod with_eol;
use with_eol::WithEOL;

pub mod with_custom_delim;
use with_custom_delim::WithCustomDelim;

/*
    We must import here to be acessible everywhere
*/

pub mod utils;

////
#[pymodule]
fn file_operations_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WithEOL>()?;
    m.add_class::<WithCustomDelim>()?;
    Ok(())
}
