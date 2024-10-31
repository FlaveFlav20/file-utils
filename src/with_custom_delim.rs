use pyo3::prelude::*;

use std::collections::VecDeque;

use crate::read_delim::ReadDelimiter;
use crate::utils::convert_queue_to_vec;

#[pyclass]
pub struct WithCustomDelim {}

#[pymethods]
impl WithCustomDelim {
    #[staticmethod]
    pub fn head(file: String, n: usize, delimiter: Vec<String>, remove_empty_string: bool) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter).expect("Unable to initialize delimiter");

        if n == 0 {
            return result;
        }
        
        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string() == "".to_string() {
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
    pub fn between(file: String, n1: usize, n2: usize, delimiter: Vec<String>, remove_empty_string: bool) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter).expect("Unable to initialize delimiter");

        let mut counter: usize = 0;
        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string() == "".to_string() {
                continue;
            }
            if counter >= n1 && counter < n2 {
                result.push(read.line.to_string());
            }
            else if counter > n2 {
                break;
            }
            counter += 1;
        }
        result
    }

    #[staticmethod]
    pub fn tail(file: String, n: usize, delimiter: Vec<String>, remove_empty_string: bool) -> Vec<String> {
        let mut result: VecDeque<String> = VecDeque::with_capacity(n);
        let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter).expect("Unable to initialize delimiter");
    
        while read.read().expect("Unable to read delimiter") == true {
            if remove_empty_string && read.line.to_string() == "".to_string() {
                continue;
            }
            if result.len() == n {
                result.remove(0);
            }
            result.push_back(read.line.to_string());
        }
        convert_queue_to_vec(result)
    }
}