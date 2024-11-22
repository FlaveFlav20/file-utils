use pyo3::prelude::*;

use regex::Regex;

use crate::utils::read_delim::ReadDelimiter;
use crate::utils::utils::{init_regex, check_regex};

#[pyclass]
pub struct WithCustomDelims {}

#[pymethods]
impl WithCustomDelims {

    #[staticmethod]
    #[pyo3(signature = (path, n, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict=true, buffer_size=1024))]
    pub fn head(
        path: String,
        n: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if n == 0 {
            return result;
        }

        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        let mut read: ReadDelimiter = ReadDelimiter::new(path, delimiter, buffer_size)
            .expect("Unable to initialize delimiter");

        let mut count: usize = 0;
        while read.read().expect("Unable to read delimiter") == true {
            count += 1;
            if remove_empty_string && read.line.to_string().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(&read.line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(&read.line, &re_pass) {
                continue;
            }
            if restrict && (count - 1) >= n {
                break;
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
        let mut result: Vec<String> = Vec::new();

        if n == 0 {
            return result;
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
            result.push(read.line.to_string());
        }
        result
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
