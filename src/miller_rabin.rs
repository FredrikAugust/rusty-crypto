extern crate rand;

use rand::{thread_rng, Rng};

// PP as in Potentially Prime
pub fn miller_rabin_test<'a>(n: i128) -> &'a str {
    // Well, obviously.
    if n & 0b1 == 0 {
        return "composite";
    }

    if n < 3 {
        return "conclusive";
    }

    // Determine the k in the n-1=2^kq representation
    let mut k = 0;

    // We need a mutable version
    let mut _n = n - 1;

    // Divide by two until we have an odd number
    while _n & 0b1 != 1 {
        _n = _n >> 1;
        k += 1;
    }

    let q = (n - 1) / 2i128.pow(k);

    // Select random number
    let mut rng = thread_rng();

    let a = rng.gen_range(2, n - 1);

    use std::convert::TryInto;

    if a.pow(q.try_into().unwrap()) % n == 1 {
        return "inconclusive";
    }

    // Exclusive, so [0, k-1]
    for j in 0..k {
        let exponent: i128 = 2i128.pow(j);
        if a.pow((exponent * q).try_into().unwrap()) % n == n - 1 {
            return "inconclusive";
        }
    }

    return "composite";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn miller_rabin_test_test() {
        for _ in 0..100 {
            assert_eq!(miller_rabin_test(29), "inconclusive");
        }
    }

    #[test]
    fn miller_rabin_catches_non_prime_test() {
        let mut results: Vec<&str> = vec![];
        for _ in 0..100 {
            results.push(miller_rabin_test(27));
        }
        assert_eq!(results.contains(&"composite"), true);
    }
}
