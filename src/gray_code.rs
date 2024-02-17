fn find_msb_pos(mut a: u32) -> usize {
    let mut msg_pos = 0;

    if a == 0 {
        return 0;
    }

    a >>= 1;
    while a != 0 {
        a >>= 1;
        msg_pos += 1;
    }
    msg_pos
}

pub fn gray_code(a: u32) -> u32 {
    let mut i = find_msb_pos(a) as i32;
    let mut rslt = a & (1 << i); /* Record the MSG as it is */

    i -= 1;
    while i >= 0 {
        let pre_bit = a & (1 << (i + 1));
        let curr_bit = a & (1 << i);

        rslt |= curr_bit ^ (pre_bit >> 1); /* XOR the precedent bit with the current bit */

        i -= 1;
    }

    rslt
}

#[cfg(test)]
mod tests {
    use super::find_msb_pos;
    use super::gray_code;

    #[test]
    fn test_find_msb_pos() {
        assert_eq!(find_msb_pos(0b1), 0);
        assert_eq!(find_msb_pos(0b0), 0);
        assert_eq!(find_msb_pos(0b1000), 3);
        assert_eq!(find_msb_pos(0b1000_000), 6);
        assert_eq!(find_msb_pos(0b0101_1010_110), 9);
    }

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(8), 12);
        assert_eq!(gray_code(732_819), 0b11101011100111011010);
        assert_eq!(gray_code(u32::MAX), 0b10000000000000000000000000000000);
    }
}
