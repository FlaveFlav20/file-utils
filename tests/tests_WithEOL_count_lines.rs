use file_operations_lib::with_eol::WithEOL;
use std::process::Command;

fn convert_string_to_list(str: String) -> Vec<String> {
    let mut convert: Vec<String> = str.split('\n').map(|e| e.to_string()).collect();
    if convert.len() == 1 && convert[0] == "".to_string() {
        convert = Vec::new();
    }

    /*
        if we have "blablabla\n" the tail command return ["blablabla", ""], so we must remove it
    */
    if convert.len() > 1 && convert[convert.len() - 1].is_empty() {
        convert.remove(convert.len() - 1);
    }
    convert
}

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

        let check_count: usize = WithEOL::count_lines(
            PATH.to_string(),
            false,
            false,
            false,
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(count_ref.len(), check_count);
    }
}
