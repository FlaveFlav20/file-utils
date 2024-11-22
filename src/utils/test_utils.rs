pub fn cmp_vector(vec1: Vec<String>, vec2: Vec<String>) -> () {
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

pub fn convert_string_to_list(str: String) -> Vec<String> {
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
