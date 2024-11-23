pub mod byte {
    pub fn no(mut c: u8) -> u8 {
        for i in 0..8 {
            if c & (0b10000000 >> i) != 0 {
                c &= !(0b10000000 >> i);
            } else {
                c |= 0b10000000 >> i;
            }
        }
        c
    }
}