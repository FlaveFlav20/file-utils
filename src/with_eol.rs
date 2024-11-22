use pyo3::prelude::*;

use regex::Regex;
use std::fs::read_to_string;

use crate::utils::utils::{init_regex, check_regex, restrict_remove_tail};

#[pyclass]
pub struct WithEOL {}

#[pymethods]
impl WithEOL {
    #[staticmethod]
    #[pyo3(signature = (path, n, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict=true))]
    pub fn head(
        path: String,
        n: usize,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if n == 0 {
            return result;
        }

        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        let mut count: usize = 0;
        for line in read_to_string(path).unwrap().lines() {
            count += 1;
            if remove_empty_string && line.to_string().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(line, &re_pass) {
                continue;
            }
            if restrict && (count - 1) >= n {
                break;
            }
            result.push(line.to_string());
            if result.len() >= n {
                break;
            }
        }
        result
    }

    #[staticmethod]
    #[pyo3(signature = (path, n1, n2, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict=true))]
    pub fn between(
        path: String,
        n1: usize,
        n2: usize,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        let mut count_lines: usize = 0;
        let mut count_elems: usize = 0;
        for line in read_to_string(path).unwrap().lines() {
            count_lines += 1;
            if remove_empty_string && line.to_string().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(line, &re_pass) {
                continue;
            }
            count_elems += 1;

            if restrict && count_lines > n2 {
                break;
            } else if restrict && count_lines >= n1 {
                result.push(line.to_string());
            } else if !restrict && count_elems > n2 {
                break;
            } else if !restrict && count_elems >= n1 {
                result.push(line.to_string());
            }
        }
        result
    }

    #[staticmethod]
    #[pyo3(signature = (path, n, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict=true))]
    pub fn tail(
        path: String,
        n: usize,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut restrict_index: Vec<usize> = Vec::new();
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        if n == 0 {
            return result;
        }

        let mut count: usize = 0;
        for line in read_to_string(path).unwrap().lines() {
            count += 1;
            if remove_empty_string && line.to_string().trim().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(line, &re_pass) {
                continue;
            }
            if result.len() == n {
                result.remove(0);
            }
            if restrict && restrict_index.len() == n {
                restrict_index.remove(0);
            }
            result.push(line.to_string());
            if restrict {
                restrict_index.push(count);
            }
        }
        if restrict {
            result = restrict_remove_tail(result, restrict_index, count, n);
        }
        result
    }

    #[staticmethod]
    #[pyo3(signature = (path, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new()))]
    pub fn parse(
        path: String,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        for line in read_to_string(path).unwrap().lines() {
            if remove_empty_string && line.to_string().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(line, &re_pass) {
                continue;
            }
            result.push(line.to_string());
        }
        result
    }

    #[staticmethod]
    #[pyo3(signature = (path, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new()))]
    pub fn count_lines(
        path: String,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
    ) -> usize {
        let mut res: usize = 0;
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        for line in read_to_string(path).unwrap().lines() {
            if remove_empty_string && line.to_string().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(line, &re_pass) {
                continue;
            }
            res += 1;
        }
        res
    }
}
