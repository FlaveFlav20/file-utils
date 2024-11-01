use pyo3::prelude::*;

use std::collections::VecDeque;

use crate::read_delim::ReadDelimiter;
use crate::utils::convert_queue_to_vec;

#[pyclass]
pub struct WithCustomDelims {}

#[pymethods]
impl WithCustomDelims {
    #[staticmethod]
    #[pyo3(signature = (file, n, delimiter, remove_empty_string=false, buffer_size = 1024))]
    pub fn head(
        file: String,
        n: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter, buffer_size)
            .expect("Unable to initialize delimiter");

        if n == 0 {
            return result;
        }

        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string().is_empty() {
                continue;
            }
            result.push(read.line.to_string());
            if result.len() >= n {
                break;
            }
        }
        result
    }

    #[staticmethod]
    #[pyo3(signature = (file, n1, n2, delimiter, remove_empty_string=false, buffer_size = 1024))]
    pub fn between(
        file: String,
        n1: usize,
        n2: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter, buffer_size)
            .expect("Unable to initialize delimiter");

        let mut counter: usize = 1;
        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string().is_empty() {
                continue;
            }
            if counter > n2 {
                break;
            } else if counter >= n1 {
                result.push(read.line.to_string());
            }
            counter += 1;
        }
        result
    }

    #[staticmethod]
    #[pyo3(signature = (file, n, delimiter, remove_empty_string=false, buffer_size = 1024))]
    pub fn tail(
        file: String,
        n: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        let mut result: VecDeque<String> = VecDeque::with_capacity(n);

        if n == 0 {
            return convert_queue_to_vec(result);
        }

        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter, buffer_size)
            .expect("Unable to initialize delimiter");

        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string().is_empty() {
                continue;
            }
            if result.len() == n {
                result.remove(0);
            }
            result.push_back(read.line.to_string());
        }
        convert_queue_to_vec(result)
    }

    #[staticmethod]
    #[pyo3(signature = (file, delimiter, remove_empty_string=false, buffer_size = 1024))]
    pub fn count_lines(
        file: String,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        buffer_size: usize,
    ) -> usize {
        let mut res: usize = 0;
        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter, buffer_size)
            .expect("Unable to initialize delimiter");
        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string().is_empty() {
                continue;
            }
            res += 1;
        }
        res
    }
}
