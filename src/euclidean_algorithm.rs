pub fn euclidean_algorithm(a: i32, b: i32) -> i32 {
    let mut remainders: Vec<i32> = vec![a, b];
    let mut k = 1;

    while remainders[k] != 0 {
        let quotient = (remainders[k - 1] / remainders[k]) as f32;
        let quotient = quotient.floor() as i32;
        remainders.push(remainders[k - 1] - quotient * remainders[k]);
        println!(
            "k-1={:5} | k={:5} | q={:4}",
            remainders[k - 1],
            remainders[k],
            quotient
        );
        k += 1;
    }

    return remainders[k - 1];
}

pub fn find_modular_inverse(a: i32, n: i32) -> i32 {
    if euclidean_algorithm(a, n) != 1 {
        println!(
            "Can't find modular inverse if the numbers aren't relatively prime. a={}, n={}",
            a, n
        );
    }

    // We will do this with vectors, as that makes it a bit clearer IMO.
    let mut r = vec![n, a];
    let mut p = vec![0, 1];
    let mut q: Vec<i32> = vec![];

    while r.last().unwrap() != &0i32 {
        q.push(r[r.len() - 2] / r[r.len() - 1]);
        r.push(r[r.len() - 2] - r[r.len() - 1] * q.last().unwrap());
        p.push(p[p.len() - 2] - p[p.len() - 1] * q[q.len() - 1] % n);
        println!("{:?} {:?} {:?}", q, r, p);
    }

    // Rust doesn't do modulo, only remainder
    let result = p[p.len() - 2] % n;

    if result < 0 {
        return (result % n) + n;
    } else {
        return result % n;
    }
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
    fn test_bezout_coefficients() {
        assert_eq!(bezout_coefficients(240, 46), (-9, 47));
    }

    #[test]
    fn test_inverse_modulo() {
        assert_eq!(find_modular_inverse(3, 31), 21);
    }
}
