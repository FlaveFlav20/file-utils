use file_utils_operations_lib::with_eol::WithEOL;
use std::process::Command;

use file_utils_operations_lib::utils::test_utils::{cmp_vector, convert_string_to_list};

static PATH: &str = "./tests_files/DDHC.txt";

#[cfg(test)]
mod tests_with_eol_tail {
    use super::*;

    #[test]
    fn tail_n_10_valid_remove_empty_string_false() {
        let len: usize = 10;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("tail ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let tail_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let tail_ref: Vec<String> = convert_string_to_list(tail_ref_str);

        let check_tail: Vec<String> =
            WithEOL::tail(PATH.to_string(), len, false, Vec::new(), Vec::new(), false);

        cmp_vector(tail_ref, check_tail);
    }

    #[test]
    fn tail_n_1_valid_remove_empty_string_false() {
        let len: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("tail ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let tail_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let tail_ref: Vec<String> = convert_string_to_list(tail_ref_str);

        let check_tail: Vec<String> =
            WithEOL::tail(PATH.to_string(), len, false, Vec::new(), Vec::new(), false);
        cmp_vector(tail_ref, check_tail);
    }

    #[test]
    fn tail_n_0_valid_remove_empty_string_false() {
        let len: usize = 0;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("tail ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let tail_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let tail_ref: Vec<String> = convert_string_to_list(tail_ref_str);

        let check_tail: Vec<String> =
            WithEOL::tail(PATH.to_string(), len, false, Vec::new(), Vec::new(), false);
        cmp_vector(tail_ref, check_tail);
    }

    #[test]
    fn tail_n_10_valid_remove_empty_string_true() {
        let len: usize = 10;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | tail -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let tail_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let tail_ref: Vec<String> = convert_string_to_list(tail_ref_str);

        let check_tail: Vec<String> =
            WithEOL::tail(PATH.to_string(), len, true, Vec::new(), Vec::new(), false);

        /*for i in 0..tail_ref.len() {
            println!("{}$;{}$", tail_ref[i], check_tail[i]);
        }*/

        cmp_vector(tail_ref, check_tail);
    }

    #[test]
    fn tail_n_1_valid_remove_empty_string_true() {
        let len: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | tail -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let tail_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let tail_ref: Vec<String> = convert_string_to_list(tail_ref_str);

        let check_tail: Vec<String> =
            WithEOL::tail(PATH.to_string(), len, true, Vec::new(), Vec::new(), false);
        cmp_vector(tail_ref, check_tail);
    }

    #[test]
    fn tail_n_0_valid_remove_empty_string_true() {
        let len: usize = 0;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | tail -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let tail_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let tail_ref: Vec<String> = convert_string_to_list(tail_ref_str);

        let check_tail: Vec<String> =
            WithEOL::tail(PATH.to_string(), len, true, Vec::new(), Vec::new(), false);

        cmp_vector(tail_ref, check_tail);
    }
}
