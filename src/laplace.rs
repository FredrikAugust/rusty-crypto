#[path = "matrix.rs"]
mod matrix;

use matrix::submatrix;

pub fn laplace(b: &Vec<Vec<i32>>) -> i32 {
    match (b.len(), b[0].len()) {
        (2, 2) => {
            return b[0][0] * b[1][1] - b[0][1] * b[1][0];
        }
        (3, 3) => {
            let mut acc = 0;
            for i in 0..3 {
                let submatrix = submatrix((i, 0), b.to_vec());

                let intermediary = *b.get(0).unwrap().get(i).unwrap() * laplace(&submatrix);

                if i & 0b1 == 1 {
                    acc -= intermediary;
                } else {
                    acc += intermediary;
                }
            }

            return acc;
        }
        (_, _) => panic!("Not implemented."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laplace_two_by_two() {
        assert_eq!(laplace(&vec![vec![4, 6], vec![3, 8]]), 14);
    }

    #[test]
    fn test_laplace_three_by_three() {
        assert_eq!(
            laplace(&vec![vec![6, 1, 1], vec![4, -2, 5], vec![2, 8, 7]]),
            -306
        );
        assert_eq!(
            laplace(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            0
        );
    }
}
