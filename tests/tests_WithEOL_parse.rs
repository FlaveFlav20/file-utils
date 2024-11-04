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
mod tests_with_eol_parse {
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
        let check_parse: Vec<String> = WithEOL::parse(
            PATH.to_string(),
            false,
            Vec::new(),
            Vec::new(),
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
        let check_parse: Vec<String> =
            WithEOL::parse(PATH.to_string(), true, Vec::new(), Vec::new());

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
        let check_parse: Vec<String> =
            WithEOL::parse(PATH.to_string(), false, reg, Vec::new());

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
        let check_parse: Vec<String> =
            WithEOL::parse(PATH.to_string(), true, reg, Vec::new());

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
        let check_parse: Vec<String> =
            WithEOL::parse(PATH.to_string(), false, Vec::new(), reg);

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
        let check_parse: Vec<String> =
            WithEOL::parse(PATH.to_string(), true, Vec::new(), reg);

        cmp_vector(parse_ref, check_parse);
    }
}
