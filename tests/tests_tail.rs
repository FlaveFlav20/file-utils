use file_operations_lib::tail::{tail_string, tail_};

fn cmp_vector(vec1: Vec<String>, vec2: Vec<String>) -> (){
    assert_eq!(vec1.len(), vec2.len(), "Not the same len");

    for i in 0..vec1.len() {
        assert_eq!(vec1[i], vec2[i], "Not the same! i: {}; vec1[i]: \"{}\"; vec2[i]: \"{}\"", i, vec1[i], vec2[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tail_to_list_valid() {
        let tail_ref: Vec<String> = Vec::from([
            "The caged bird sings".to_string(),
            "with a fearful trill".to_string(),
            "of things unknown".to_string(),
            "but longed for still".to_string(),
            "and his tune is heard".to_string(),
            "on the distant hill   ".to_string(),
            "for the caged bird   ".to_string(),
            "sings of freedom.".to_string(),
        ]);

        let check_tail: Vec<String> = tail_("./tests/files/cages birds.txt".to_string(), tail_ref.len());
        cmp_vector(tail_ref, check_tail);
    }

    #[test]
    fn tail_to_list_valid_one() {
        let tail_ref: Vec<String> = Vec::from([
            "sings of freedom.".to_string(),
        ]);

        let check_tail: Vec<String> = tail_("./tests/files/cages birds.txt".to_string(), tail_ref.len());
        cmp_vector(tail_ref, check_tail);
    }
}