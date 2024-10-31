use pyo3::prelude::*;

use std::collections::VecDeque;
use std::fs::read_to_string;

use crate::utils::convert_queue_to_vec;

#[pyclass]
pub struct WithEOL {}

#[pymethods]
impl WithEOL {
    #[staticmethod]
    pub fn head(file: String, n: usize, remove_empty_string: bool) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if n == 0 {
            return result;
        }
        
        for line in read_to_string(file).unwrap().lines() {
            if remove_empty_string && line.to_string() == "".to_string() {
                continue;
            }
            result.push(line.to_string());
            if result.len() >= n {
                break;
            }
        }
        result
    }

    #[staticmethod]
    pub fn between(file: String, n1: usize, n2: usize, remove_empty_string: bool) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let mut counter: usize = 0;
        for line in read_to_string(file).unwrap().lines() {
            if remove_empty_string && line.to_string() == "".to_string() {
                continue;
            }
            if counter > n1 && counter < n2 {
                result.push(line.to_string());
            }
            else if counter > n2 {
                break;
            }
            counter += 1;
        }
        result
    }

    #[staticmethod]
    pub fn tail(file: String, n: usize, remove_empty_string: bool) -> Vec<String> {
        let mut result: VecDeque<String> = VecDeque::with_capacity(n);
    
        for line in read_to_string(file).unwrap().lines() {
            if remove_empty_string && line.to_string() == "".to_string() {
                continue;
            }
            if result.len() == n {
                result.remove(0);
            }
            result.push_back(line.to_string());
        }
    
        convert_queue_to_vec(result)
    }
}