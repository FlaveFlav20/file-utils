use file_utils_operations_lib::utils::byte::byte;

mod tests_byte {
    use super::*;

    #[test]
    fn test_all_zero() {
        let test: u8 = 0b00000000;
        let expected: u8 = 0b11111111;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }

    #[test]
    fn test_all_one() {
        let test: u8 = 0b11111111;
        let expected: u8 = 0b00000000;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }

    #[test]
    fn test_one_begin() {
        let test: u8 = 0b10000000;
        let expected: u8 = 0b01111111;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }

    #[test]
    fn test_one_end() {
        let test: u8 = 0b00000001;
        let expected: u8 = 0b11111110;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }

    #[test]
    fn test_one_middle() {
        let test: u8 = 0b00010000;
        let expected: u8 = 0b11101111;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }

    #[test]
    fn test_mixt_1() {
        let test: u8 = 0b01100110;
        let expected: u8 = 0b10011001;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }

    #[test]
    fn test_mixt_2() {
        let test: u8 = 0b10010110;
        let expected: u8 = 0b01101001;
        assert_eq!(byte::no(test), expected, "Expected {:#b}; Got {:#b}", expected, byte::no(test)); 
    }
}