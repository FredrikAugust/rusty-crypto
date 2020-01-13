pub fn euclidean_algorithm(a: u32, b: u32) -> u32 {
    let mut r: Vec<u32> = vec![a, b];
    let mut k = 1;

    while r[k] != 0 {
        let q = (r[k - 1] / r[k]) as f32;
        let q = q.floor() as u32;
        r.push(r[k - 1] - q * r[k]);
        k += 1;
    }

    k -= 1;

    return r[k];
}

// Computes Bézout coefficients by using the extended Euclidean algorithm
pub fn bezout_coefficients(a: i32, b: i32) -> (i32, i32) {
    let [mut s, mut t, mut r] = [0, 1, b];
    // As we actually only care about the newest (k) and the previous (k-1)
    // values, we don't need a vector, we can just use two variables.
    let [mut old_s, mut old_t, mut old_r] = [1, 0, a];

    while r != 0 {
        let q = (old_r / r) as f32;
        let q = q.floor() as i32;

        let new_r = old_r - q * r;
        old_r = r;
        r = new_r;

        let new_s = old_s - q * s;
        old_s = s;
        s = new_s;

        let new_t = old_t - q * t;
        old_t = t;
        t = new_t;
    }

    return (old_s, old_t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_algorithm() {
        assert_eq!(euclidean_algorithm(12, 24), 12);
        assert_eq!(euclidean_algorithm(6, 24), 6);
        assert_eq!(euclidean_algorithm(3, 30), 3);
    }

    #[test]
    fn test_extended_euclidean_algorithm() {
        assert_eq!(bezout_coefficients(240, 46), (-9, 47));
    }
}
