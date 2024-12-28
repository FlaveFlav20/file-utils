use regex::Regex;

pub fn init_regex(list_str: Vec<String>) -> Vec<Regex> {
    let mut res: Vec<Regex> = Vec::new();
    if list_str.len() > 0 {
        for i in 0..list_str.len() {
            res.push(Regex::new(&list_str[i]).unwrap())
        }
    }
    res
}

pub fn check_regex(to_check: &str, list_regex: &Vec<Regex>) -> bool {
    for i in 0..list_regex.len() {
        if list_regex[i].is_match(to_check) {
            return true;
        }
    }
    false
}

pub fn restrict_remove_tail(
    mut list: Vec<String>,
    list_index: Vec<usize>,
    count: usize,
    n: usize,
) -> Vec<String> {
    for i in 0..list_index.len() {
        if count > n && list_index[i] < (count - n) {
            list.remove(0);
        } else {
            break;
        }
    }
    list
}
