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
mod tests_with_eol_between {
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

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithEOL::between(
            PATH.to_string(),
            n1,
            n2,
            false,
            false,
            false,
            "".to_string(),
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

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> = WithEOL::between(
            PATH.to_string(),
            n1,
            n2,
            false,
            false,
            false,
            "".to_string(),
        );

        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_5_n2_4_valid_remove_empty_string_false() {
        let n1: usize = 5;
        let n2: usize = 4;
        let between_ref: Vec<String> = Vec::new();
        let check_between: Vec<String> = WithEOL::between(
            PATH.to_string(),
            n1,
            n2,
            false,
            false,
            false,
            "".to_string(),
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

        let between_ref: Vec<String> = convert_string_to_list(between_ref_str);
        let check_between: Vec<String> =
            WithEOL::between(PATH.to_string(), n1, n2, true, false, false, "".to_string());

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

        let check_between: Vec<String> =
            WithEOL::between(PATH.to_string(), n1, n2, true, false, false, "".to_string());
        cmp_vector(between_ref, check_between);
    }

    #[test]
    fn between_n1_5_n2_4_valid_remove_empty_string_true() {
        let n1: usize = 5;
        let n2: usize = 4;
        let between_ref: Vec<String> = Vec::new();
        let check_between: Vec<String> =
            WithEOL::between(PATH.to_string(), n1, n2, true, false, false, "".to_string());
        cmp_vector(between_ref, check_between);
    }
}
