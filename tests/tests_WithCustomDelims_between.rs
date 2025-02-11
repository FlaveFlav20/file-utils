use file_utils_lib::with_custom_delims::WithCustomDelims;
use std::process::Command;

use file_utils_lib::utils::test_utils::{
    cmp_vector, convert_string_to_list, get_custom_delims,
};

static PATH: &str = "./tests_files/DDHC.txt";
static PATH_DELIMS: &str = "./tests_files/DDHC_custom_delims.txt";

#[cfg(test)]
mod tests_withcustomdelim_between {
    use super::*;

    #[test]
    fn between_n1_1_n2_2_valid_remove_empty_string_false() {
        let n1: usize = 1;
        let n2: usize = 2;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed -n '".to_string()
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p' "
                        + PATH,
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let delims: Vec<String> = get_custom_delims();

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_1_n2_1_valid_remove_empty_string_false() {
        let n1: usize = 1;
        let n2: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed -n '".to_string()
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p' "
                        + PATH,
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let delims: Vec<String> = get_custom_delims();

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_5_n2_4_valid_remove_empty_string_false() {
        let n1: usize = 5;
        let n2: usize = 4;
        let between_ref: Vec<String> = Vec::new();

        let delims: Vec<String> = get_custom_delims();

        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_1_n2_2_valid_remove_empty_string_true() {
        let n1: usize = 1;
        let n2: usize = 2;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed '/^$/d' ".to_string()
                        + PATH
                        + " | sed -n '"
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p'",
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let delims: Vec<String> = get_custom_delims();

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_1_n2_1_valid_remove_empty_string_true() {
        let n1: usize = 1;
        let n2: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed '/^$/d' ".to_string()
                        + PATH
                        + " | sed -n '"
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p'",
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );
        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_5_n2_4_valid_remove_empty_string_true() {
        let n1: usize = 5;
        let n2: usize = 4;
        let between_ref: Vec<String> = Vec::new();

        let delims: Vec<String> = get_custom_delims();

        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );
        cmp_vector(between_ref, check_between);
    }

    ///////

    #[test]
    fn between_n1_1_n2_2_valid_remove_empty_string_false_little_buffer() {
        let n1: usize = 1;
        let n2: usize = 2;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed -n '".to_string()
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p' "
                        + PATH,
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let delims: Vec<String> = get_custom_delims();

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_1_n2_1_valid_remove_empty_string_false_little_buffer() {
        let n1: usize = 1;
        let n2: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed -n '".to_string()
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p' "
                        + PATH,
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let delims: Vec<String> = get_custom_delims();

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_5_n2_4_valid_remove_empty_string_false_little_buffer() {
        let n1: usize = 5;
        let n2: usize = 4;
        let between_ref: Vec<String> = Vec::new();

        let delims: Vec<String> = get_custom_delims();

        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_1_n2_2_valid_remove_empty_string_true_little_buffer() {
        let n1: usize = 1;
        let n2: usize = 2;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed '/^$/d' ".to_string()
                        + PATH
                        + " | sed -n '"
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p'",
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let delims: Vec<String> = get_custom_delims();

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_1_n2_1_valid_remove_empty_string_true_little_buffer() {
        let n1: usize = 1;
        let n2: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(
                    "sed '/^$/d' ".to_string()
                        + PATH
                        + " | sed -n '"
                        + &n1.to_string()
                        + ","
                        + &n2.to_string()
                        + " p'",
                )
                .output()
                .expect("failed to execute process")
        };

        let between_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );
        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_5_n2_4_valid_remove_empty_string_true_little_buffer() {
        let n1: usize = 5;
        let n2: usize = 4;
        let between_ref: Vec<String> = Vec::new();

        let delims: Vec<String> = get_custom_delims();

        let check_between: Vec<String> = WithCustomDelims::between(
            PATH_DELIMS.to_string(),
            n1,
            n2,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );
        cmp_vector(between_ref, check_between);
    }
}
