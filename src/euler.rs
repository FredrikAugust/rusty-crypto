#[path = "euclidean_algorithm.rs"]
mod euclidean_algorithm;

use euclidean_algorithm::euclidean_algorithm;

pub fn euler_totient(n: i32) -> i32 {
    let mut total = 0i32;

    for i in 1..n {
        if euclidean_algorithm(i, n) == 1 {
            total += 1;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_totient() {
        assert_eq!(euler_totient(37), 36);
        assert_eq!(euler_totient(35), 24);
    }
}
