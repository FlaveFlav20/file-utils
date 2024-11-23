use file_utils_operations_lib::utils::non_ascii_char::non_ascii_char;

mod tests_non_ascii_char {
    use super::*;

    #[test]
    fn test_ascii_char() {
        for c in 0..128 {
            assert_eq!(
                non_ascii_char::check_non_ascii(c as u8),
                false,
                "The character {} is detected as non ascii char...",
                c as u8 as char
            );
        }
    }

    #[test]
    fn test_non_ascii_char() {
        for c in 128..255 {
            assert_eq!(
                non_ascii_char::check_non_ascii(c as u8),
                true,
                "The character {} is detected as ascii char...",
                c as u8 as char
            );
        }
    }

    #[test]
    fn test_check_number_bytes_begin_one() {
        let valid_one: u8 = 0b10000000;
        let expect = 1;
        assert_eq!(
            non_ascii_char::check_number_bytes_begin(valid_one),
            expect,
            "Expected {}; Got {}",
            expect,
            non_ascii_char::check_number_bytes_begin(valid_one)
        );
    }

    #[test]
    fn test_check_number_bytes_begin_two() {
        let valid_two: u8 = 0b11000000;
        let expect = 2;
        assert_eq!(
            non_ascii_char::check_number_bytes_begin(valid_two),
            expect,
            "Expected {}; Got {}",
            expect,
            non_ascii_char::check_number_bytes_begin(valid_two)
        );
    }

    #[test]
    fn test_check_number_bytes_begin_three() {
        let valid_three: u8 = 0b11100000;
        let expect = 3;
        assert_eq!(
            non_ascii_char::check_number_bytes_begin(valid_three),
            expect,
            "Expected {}; Got {}",
            expect,
            non_ascii_char::check_number_bytes_begin(valid_three)
        );
    }

    #[test]
    fn test_check_number_bytes_begin_four() {
        let valid_four: u8 = 0b11110000;
        let expect = 4;
        assert_eq!(
            non_ascii_char::check_number_bytes_begin(valid_four),
            expect,
            "Expected {}; Got {}",
            expect,
            non_ascii_char::check_number_bytes_begin(valid_four)
        );
    }

    #[test]
    fn test_check_number_bytes_begin_invalid() {
        let invalid: u8 = 0b11111000;
        let expect = -1;
        assert_eq!(
            non_ascii_char::check_number_bytes_begin(invalid),
            expect,
            "Expected {}; Got {}",
            expect,
            non_ascii_char::check_number_bytes_begin(invalid)
        );
    }
}
