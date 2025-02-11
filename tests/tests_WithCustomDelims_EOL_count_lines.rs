use file_utils_lib::with_custom_delims::WithCustomDelims;
use std::process::Command;

use file_utils_lib::utils::test_utils::convert_string_to_list;

static PATH: &str = "./tests_files/DDHC.txt";

#[cfg(test)]
mod tests_withcustomdelim_eol_count_lines {
    use super::*;

    #[test]
    fn count_lines_basic() {
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

        let count_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let count_ref: Vec<String> = convert_string_to_list(count_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_count: usize = WithCustomDelims::count_lines(
            PATH.to_string(),
            delims,
            false,
            Vec::new(),
            Vec::new(),
            1024,
        );

        assert_eq!(count_ref.len(), check_count);
    }

    /////

    #[test]
    fn count_lines_basic_little_buffer() {
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

        let count_ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let count_ref: Vec<String> = convert_string_to_list(count_ref_str);

        let mut delims: Vec<String> = Vec::new();
        delims.push(String::from('\n'));

        let check_count: usize = WithCustomDelims::count_lines(
            PATH.to_string(),
            delims,
            false,
            Vec::new(),
            Vec::new(),
            4,
        );

        assert_eq!(count_ref.len(), check_count);
    }
}
