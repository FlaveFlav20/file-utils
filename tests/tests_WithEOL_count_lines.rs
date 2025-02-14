use file_utils_lib::with_eol::WithEOL;
use std::process::Command;

use file_utils_lib::utils::test_utils::convert_string_to_list;

static PATH: &str = "./tests_files/DDHC.txt";

#[cfg(test)]
mod tests_with_eol_count_lines {
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

        let check_count: usize =
            WithEOL::count_lines(PATH.to_string(), false, Vec::new(), Vec::new());

        assert_eq!(count_ref.len(), check_count);
    }
}
