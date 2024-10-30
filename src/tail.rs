use pyo3::prelude::*;

use std::collections::VecDeque;
use std::fs::read_to_string;

use crate::read_delim::ReadDelimiter;

fn convert_queue_to_string(queue: VecDeque<String>) -> String {
    if queue.len() == 0 {
        return "".to_string();
    }

    let mut res: String = queue[0].clone();
    let new_line = "\n".to_string();

    for i in 1..queue.len() {
        res += &(new_line.clone() + &queue[i]);
    }
    res
}

fn convert_queue_to_vec(queue: VecDeque<String>) -> Vec<String> {
    let mut res = Vec::new();

    for i in 0..queue.len() {
        res.push(queue[i].clone())
    }
    res
}

#[pyfunction]
pub fn tail_string(file: String, n: usize) -> String {
    let mut result: VecDeque<String> = VecDeque::with_capacity(n);

    for line in read_to_string(file).unwrap().lines() {
        if result.len() == n {
            result.remove(0);
        }
        result.push_back(line.to_string())
    }

    convert_queue_to_string(result)
}

#[pyfunction]
pub fn tail_(file: String, n: usize) -> Vec<String> {
    let mut result: VecDeque<String> = VecDeque::with_capacity(n);

    for line in read_to_string(file).unwrap().lines() {
        if result.len() == n {
            result.remove(0);
        }
        result.push_back(line.to_string());
    }

    convert_queue_to_vec(result)
}

#[pyfunction]
pub fn tail_string_delim(file: String, n: usize, delimiter: String) -> String {
    let mut result: VecDeque<String> = VecDeque::with_capacity(n);
    let mut read: ReadDelimiter = ReadDelimiter::new(file, delimiter).expect("Unable to initialize delimiter");

    while read.read().expect("Unable to read delimiter") == true {
        if result.len() == n {
            result.remove(0);
        }
        result.push_back(read.line.to_string());
    }
    convert_queue_to_string(result)
}