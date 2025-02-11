use file_utils_lib::with_custom_delims::WithCustomDelims;
use std::process::Command;

use file_utils_lib::utils::test_utils::{cmp_vector, convert_string_to_list};

static PATH: &str = "./tests_files/DDHC.txt";

#[cfg(test)]
mod tests_withcustomdelim_eol_parse {
    use super::*;
    #[test]
    fn parse_remove_empty_string_false_keep_regex_false_pass_when_regex_false() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("cat ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> = WithCustomDelims::parse(
            PATH.to_string(),
            delims,
            false,
            Vec::new(),
            Vec::new(),
            1024,
        );

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_true_keep_regex_false_pass_when_regex_false() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("sed '/^$/d' ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, true, Vec::new(), Vec::new(), 1024);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_false_keep_regex_true_pass_when_regex_false() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("grep \"^La loi\" ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, false, reg, Vec::new(), 1024);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_true_keep_regex_true_pass_when_regex_false() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("sed '/^$/d' ".to_string() + PATH + " | grep \"^La loi\" ").to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, true, reg, Vec::new(), 1024);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_false_keep_regex_false_pass_when_regex_true() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("grep -v \"^La loi\" ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, false, Vec::new(), reg, 1024);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_true_keep_regex_false_pass_when_regex_true() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("sed '/^$/d' ".to_string() + PATH + " | grep -v \"^La loi\" ").to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, true, Vec::new(), reg, 1024);

        cmp_vector(parse_ref, check_parse);
    }

    /////
    #[test]
    fn parse_remove_empty_string_false_keep_regex_false_pass_when_regex_false_little_buffer() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("cat ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, false, Vec::new(), Vec::new(), 4);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_true_keep_regex_false_pass_when_regex_false_little_buffer() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("sed '/^$/d' ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, true, Vec::new(), Vec::new(), 4);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_false_keep_regex_true_pass_when_regex_false_little_buffer() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("grep \"^La loi\" ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, false, reg, Vec::new(), 4);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_true_keep_regex_true_pass_when_regex_false_little_buffer() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("sed '/^$/d' ".to_string() + PATH + " | grep \"^La loi\" ").to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, true, reg, Vec::new(), 4);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_false_keep_regex_false_pass_when_regex_true_little_buffer() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("grep -v \"^La loi\" ".to_string() + PATH).to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, false, Vec::new(), reg, 4);

        cmp_vector(parse_ref, check_parse);
    }

    #[test]
    fn parse_remove_empty_string_true_keep_regex_false_pass_when_regex_true_little_buffer() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(("sed '/^$/d' ".to_string() + PATH + " | grep -v \"^La loi\" ").to_string())
                .output()
                .expect("failed to execute process")
        };

        let parse_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let mut reg: Vec<String> = Vec::new();
        reg.push("^La loi".to_string());

        let parse_ref: Vec<String> = convert_string_to_list(parse_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_parse: Vec<String> =
            WithCustomDelims::parse(PATH.to_string(), delims, true, Vec::new(), reg, 4);

        cmp_vector(parse_ref, check_parse);
    }
}
