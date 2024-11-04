use file_operations_lib::with_eol::WithEOL;
use std::process::Command;

fn cmp_vector(vec1: Vec<String>, vec2: Vec<String>) -> () {
    assert_eq!(
        vec1.len(),
        vec2.len(),
        "Not the same len, vec1.len() (ref): \"{}\"; vec2.len() (to test): \"{}\"",
        vec1.len(),
        vec2.len()
    );

    for i in 0..vec1.len() {
        assert_eq!(
            vec1[i], vec2[i],
            "Not the same! i: {}; vec1[i] (ref): \"{}\"; vec2[i] (to test): \"{}\"",
            i, vec1[i], vec2[i]
        );
    }
}

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
mod tests_with_eol_head {
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

        let check_head: Vec<String> = WithEOL::head(
            PATH.to_string(),
            len,
            false,
            Vec::new(),
            Vec::new(),
            false,
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

        let check_head: Vec<String> = WithEOL::head(
            PATH.to_string(),
            len,
            false,
            Vec::new(),
            Vec::new(),
            false,
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

        let check_head: Vec<String> = WithEOL::head(
            PATH.to_string(),
            len,
            false,
            Vec::new(),
            Vec::new(),
            false,
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

        let check_head: Vec<String> = WithEOL::head(
            PATH.to_string(),
            len,
            true,
            Vec::new(),
            Vec::new(),
            false,
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

        let check_head: Vec<String> = WithEOL::head(
            PATH.to_string(),
            len,
            true,
            Vec::new(),
            Vec::new(),
            false,
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

        let check_head: Vec<String> = WithEOL::head(
            PATH.to_string(),
            len,
            true,
            Vec::new(),
            Vec::new(),
            false,
        );

        cmp_vector(head_ref, check_head);
    }
}
