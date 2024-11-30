//!
//! It using a list of delimiters to parse the file
//!

use pyo3::prelude::*;

use regex::Regex;

use crate::utils::utils::{check_regex, init_regex, restrict_remove_tail};
use read_utf8::read_utf8_delims::ReadUTF8Delims;

///
/// [WithCustomDelims]: a simple structure to call functions (they are all static)
///
#[pyclass]
pub struct WithCustomDelims {}

#[pymethods]
impl WithCustomDelims {
    ///
    /// [WithCustomDelims]\[head\]: take the first n lines. \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n (mandatory): number of lines
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the first n lines.
    /// If not, it will take the first n lines that can be taken (you can take
    /// a look at the README to have further explaination)
    ///
    /// return:
    /// - A list of string
    ///
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

        let read: ReadUTF8Delims = ReadUTF8Delims::new(path, delimiter, None, Some(buffer_size))
            .expect("Unable to initialize delimiter");

        let mut count: usize = 0;
        for line in read.into_iter() {
            count += 1;
            if remove_empty_string && line.is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(&line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(&line, &re_pass) {
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
    /// [WithCustomDelims]\[between\]: take the lines between n1 and n2 \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n1 (mandatory): the line begin
    /// - n2 (mandatory): the line end
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the lines between
    /// n1 and n2. If not, it will take the lines between n1 and n2 that can be
    /// taken (you can take a look at the README to have further explaination)
    ///
    /// return:
    /// - A list of string
    ///
    #[staticmethod]
    #[pyo3(signature = (path, n1, n2, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict = true, buffer_size = 1024))]
    pub fn between(
        path: String,
        n1: usize,
        n2: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let read: ReadUTF8Delims = ReadUTF8Delims::new(path, delimiter, None, Some(buffer_size))
            .expect("Unable to initialize delimiter");

        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        let mut count_lines: usize = 0;
        let mut count_elems: usize = 0;

        for line in read.into_iter() {
            count_lines += 1;
            if remove_empty_string && line.is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(&line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(&line, &re_pass) {
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
    /// [WithCustomDelims]\[tail\]: take the last n lines. \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n (mandatory): number of lines
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the last n lines.
    /// If not, it will take the last n lines that can be taken (you can take
    /// a look at the README to have further explaination)
    ///
    /// return:
    /// - A list of string
    ///
    #[staticmethod]
    #[pyo3(signature = (path, n, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict = true, buffer_size = 1024))]
    pub fn tail(
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
        let mut restrict_index: Vec<usize> = Vec::new();
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        if n == 0 {
            return result;
        }

        let read: ReadUTF8Delims = ReadUTF8Delims::new(path, delimiter, None, Some(buffer_size))
            .expect("Unable to initialize delimiter");

        let mut count: usize = 0;
        for line in read.into_iter() {
            count += 1;
            if remove_empty_string && line.to_string().trim().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(&line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(&line, &re_pass) {
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
    /// [WithCustomDelims]\[parse\]: take the whole file \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    ///
    /// return:
    /// - A list of string
    ///
    #[staticmethod]
    #[pyo3(signature = (path, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), buffer_size = 1024))]
    pub fn parse(
        path: String,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        buffer_size: usize,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        let read: ReadUTF8Delims = ReadUTF8Delims::new(path, delimiter, None, Some(buffer_size))
            .expect("Unable to initialize delimiter");

        for line in read.into_iter() {
            if remove_empty_string && line.is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(&line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(&line, &re_pass) {
                continue;
            }
            result.push(line.to_string());
        }
        result
    }

    ///
    /// [WithCustomDelims]\[parse\]: count the number of matched lines \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    ///
    /// return:
    /// - number of lines => usize
    ///
    #[staticmethod]
    #[pyo3(signature = (path, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), buffer_size = 1024))]
    pub fn count_lines(
        path: String,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        buffer_size: usize,
    ) -> usize {
        let mut res: usize = 0;
        let re_keep: Vec<Regex> = init_regex(regex_keep);
        let re_pass: Vec<Regex> = init_regex(regex_pass);

        let read: ReadUTF8Delims = ReadUTF8Delims::new(path, delimiter, None, Some(buffer_size))
            .expect("Unable to initialize delimiter");
        for line in read.into_iter() {
            if remove_empty_string && line.to_string().is_empty() {
                continue;
            } else if re_keep.len() > 0 && !check_regex(&line, &re_keep) {
                continue;
            } else if re_pass.len() > 0 && check_regex(&line, &re_pass) {
                continue;
            }
            res += 1;
        }
        res
    }
}
