pub fn adder(a: u32, b: u32) -> u32 {
    let mut rslt = 0;
    let mut carry = 0;

    for i in 0..32 {
        let mask = 1 << i;
        let a_bit = a & mask;
        let b_bit = b & mask;

        rslt |= a_bit ^ b_bit ^ carry;
        carry = (a_bit & b_bit) | (carry & (a_bit ^ b_bit));
        carry <<= 1;
    }

    rslt
}

#[cfg(test)]
mod tests {
    use super::adder;
    use rand::{thread_rng, Rng};

    const NBR_TEST: usize = 1_000_000;

    #[test]
    fn test_adder() {
        let mut rng = thread_rng();

        for _ in 0..NBR_TEST {
            let a = rng.gen_range(0..=u32::MAX);
            let b = rng.gen_range(0..=(u32::MAX - a));
            assert_eq!(adder(a, b), a + b);
        }
    }
}
