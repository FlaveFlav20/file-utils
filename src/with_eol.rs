//!
//! It using the end of line to parse the file
//!

use pyo3::prelude::*;

use regex::Regex;
use std::fs::read_to_string;

use crate::utils::utils::{check_regex, init_regex, restrict_remove_tail};

#[pyclass]
///
/// [WithEOL]: a simple structure to call functions (they are all static)
///
pub struct WithEOL {}

#[pymethods]
impl WithEOL {
    ///
    /// [WithEOL]\[head\]: take the first n lines. \
    /// Arguments:
    /// - path: to the file
    /// - n: number of lines
    /// - remove_empty_string (false by default in python code):: remove all
    /// string that only contains spaces
    /// - regex_keep: a list of regex to keep => put Vec::new() if
    /// you don't want this parameters
    /// - regex_pass: a list of regex to pass => put Vec::new() if
    /// you don't want this parameters
    /// - restrict(true by default in python code): if enable, it will only
    /// take the first n lines. If not, it will take the first n lines that
    /// can be taken (you can take a look at the README to have further
    /// explaination)
    ///
    /// Return:
    /// - A list of string
    ///
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

    ///
    /// [WithEOL]\[between\]: take the lines between n1 and n2 \
    /// Arguments:
    /// - path: to the file
    /// - n1: the line begin
    /// - n2: the line end
    /// - remove_empty_string (false by default in python code):: remove all
    /// string that only contains spaces
    /// - regex_keep: a list of regex to keep => put Vec::new() if
    /// you don't want this parameters
    /// - regex_pass: a list of regex to pass => put Vec::new() if
    /// you don't want this parameters
    /// - restrict(true by default in python code): if enable, it will only
    /// take the lines between n1 and n2. If not, it will take the lines
    /// between n1 and n2 that can be taken (you can take a look at the README
    ///  to have further explaination)
    ///
    /// Return:
    /// - A list of string
    ///
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

    ///
    /// [WithEOL]\[tail\]: take the last n lines. \
    /// Arguments:
    /// - path: to the file
    /// - n: number of lines
    /// - remove_empty_string (false by default in python code):: remove all
    /// string that only contains spaces
    /// - regex_keep: a list of regex to keep => put Vec::new() if
    /// you don't want this parameters
    /// - regex_pass: a list of regex to pass => put Vec::new() if
    /// you don't want this parameters
    /// - restrict(true by default in python code): if enable, it will only
    /// take the last n lines. If not, it will take the last n lines that can
    /// be taken (you can take a look at the README to have further
    /// explaination)
    ///
    /// Return:
    /// - A list of string
    ///
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

    ///
    /// [WithEOL]\[parse\]: take the whole file \
    /// Arguments:
    /// - path: to the file
    /// - remove_empty_string (false by default in python code):: remove all
    /// string that only contains spaces
    /// - regex_keep: a list of regex to keep => put Vec::new() if
    /// you don't want this parameters
    /// - regex_pass: a list of regex to pass => put Vec::new() if
    /// you don't want this parameters
    ///
    /// Return:
    /// - A list of string
    ///
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

    ///
    /// [WithEOL]\[parse\]: count the number of matched lines \
    /// Arguments:
    /// - path: to the file
    /// - remove_empty_string (false by default in python code):: remove all
    /// string that only contains spaces
    /// - regex_keep: a list of regex to keep => put Vec::new() if
    /// you don't want this parameters
    /// - regex_pass: a list of regex to pass => put Vec::new() if
    /// you don't want this parameters
    ///
    /// Return:
    /// - number of lines => usize
    ///
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
