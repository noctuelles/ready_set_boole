use crate::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut rslt: u32 = 0;

    for i in 0..32 {
        if b & (1 << i) != 0 {
            rslt = adder(rslt, (a & 0xffffffff) << i);
        }
    }

    rslt
}

#[cfg(test)]
mod tests {
    use super::multiplier;
    use rand::{thread_rng, Rng};
    const NBR_TEST: usize = 1_000_000;

    #[test]
    fn test_multiplier() {
        let mut rng = thread_rng();

        for _ in 0..NBR_TEST {
            let a = rng.gen_range(0..=u32::MAX);
            let b = rng.gen_range(0..=(u32::MAX / a));
            assert_eq!(multiplier(a, b), a * b);
        }
    }
}
