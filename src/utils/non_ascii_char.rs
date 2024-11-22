pub mod non_ascii_char {
    const CHECK_NON_ASCII: u8 = 0b10000000;
    const CHECK_LEN_2: u8 = 0b11000000;
    const CHECK_LEN_3: u8 = 0b1110000;
    const CHECK_LEN_4: u8 = 0b1111000;
    const CHECK_FOLLOW: u8 = 0b1000000;

    pub fn check_non_ascii(c: u8) -> bool {
        (c & CHECK_NON_ASCII) != 0
    }

    pub fn check_number_bytes_begin(c: u8) -> usize {
        if c & CHECK_LEN_2 != 0 {
            return 2;
        } else if c & CHECK_LEN_3 != 0 {
            return 3;
        } else if c & CHECK_LEN_4 != 0 {
            return 4;
        }
        return 1;
    }

    pub fn check_number_follow(c: u8) -> bool {
        if c & CHECK_FOLLOW != 0 {
            return true;
        }
        return false;
    }
}
