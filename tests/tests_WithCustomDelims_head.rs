use file_utils_lib::with_custom_delims::WithCustomDelims;
use std::process::Command;

use file_utils_lib::utils::test_utils::{
    cmp_vector, convert_string_to_list, get_custom_delims,
};

static PATH: &str = "./tests_files/DDHC.txt";
static PATH_DELIMS: &str = "./tests_files/DDHC_custom_delims.txt";

#[cfg(test)]
mod tests_withcustomdelim_head {
    use super::*;

    #[test]
    fn head_n_10_valid_remove_empty_string_false() {
        let len: usize = 10;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("head ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_1_valid_remove_empty_string_false() {
        let len: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("head ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );
        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_0_valid_remove_empty_string_false() {
        let len: usize = 0;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("head ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );
        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_10_valid_remove_empty_string_true() {
        let len: usize = 10;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | head -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_1_valid_remove_empty_string_true() {
        let len: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | head -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );
        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_0_valid_remove_empty_string_true() {
        let len: usize = 0;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | head -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            1024,
        );

        cmp_vector(head_ref, check_head);
    }

    ////////

    #[test]
    fn head_n_10_valid_remove_empty_string_false_little_buffer() {
        let len: usize = 10;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("head ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        println!("red:");
        for i in 0..head_ref.len() {
            println!("{}:{}", i, head_ref[i]);
        }
        println!("My:");
        for i in 0..check_head.len() {
            println!("{}:{}", i, check_head[i]);
        }

        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_1_valid_remove_empty_string_false_little_buffer() {
        let len: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("head ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );
        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_0_valid_remove_empty_string_false_little_buffer() {
        let len: usize = 0;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg((("head ".to_string() + PATH).to_string() + " -n ") + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            false,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );
        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_10_valid_remove_empty_string_true_little_buffer() {
        let len: usize = 10;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | head -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_1_valid_remove_empty_string_true_little_buffer() {
        let len: usize = 1;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | head -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );
        cmp_vector(head_ref, check_head);
    }

    #[test]
    fn head_n_0_valid_remove_empty_string_true_little_buffer() {
        let len: usize = 0;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("sed '/^$/d' ".to_string() + PATH + " | head -n " + &len.to_string())
                .output()
                .expect("failed to execute process")
        };

        let head_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let head_ref: Vec<String> = convert_string_to_list(head_ref_str);

        let delims: Vec<String> = get_custom_delims();

        let check_head: Vec<String> = WithCustomDelims::head(
            PATH_DELIMS.to_string(),
            len,
            delims,
            true,
            Vec::new(),
            Vec::new(),
            false,
            4,
        );

        cmp_vector(head_ref, check_head);
    }
}
