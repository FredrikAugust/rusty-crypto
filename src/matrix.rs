extern crate num;

pub fn laplace<T: num::Num + Copy + std::ops::Neg>(b: &Vec<Vec<T>>) -> T {
    match (b.len(), b[0].len()) {
        (2, 2) => {
            return b[0][0] * b[1][1] - b[0][1] * b[1][0];
        }
        (3, 3) => {
            let mut acc = T::zero();
            for i in 0..3 {
                let submatrix = submatrix((i, 0), b.to_vec());

                let intermediary = *b.get(0).unwrap().get(i).unwrap() * laplace(&submatrix);

                if i & 0b1 == 1 {
                    acc = acc - intermediary;
                } else {
                    acc = acc + intermediary;
                }
            }

            return acc;
        }
        (_, _) => panic!("Not implemented."),
    }
}

pub fn modulus<T: num::Num + Copy + std::cmp::PartialOrd>(matrix: &mut Vec<Vec<T>>, n: T) {
    for row in matrix {
        for cell in row {
            if cell < &mut T::from(num::zero()) {
                *cell = (*cell % n) + n;
            } else {
                *cell = *cell % n;
            }
        }
    }
}

pub fn multiply<T: num::Num + Copy>(a: Vec<Vec<T>>, b: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut c: Vec<Vec<T>> = Vec::new();

    if a[0].len() != b.len() {
        panic!("The y-axis of a must be equal to the x-axis of b.");
    }

    for i in 0..a.len() {
        c.push(Vec::new());

        for j in 0..b[0].len() {
            let mut acc = T::from(num::zero());

            for k in 0..a[0].len() {
                acc = acc + a[i][k] * b[k][j];
            }

            c[i].push(acc);
        }
    }

    return c;
}

pub fn scalar_matrix_multiplication(m: &mut Vec<Vec<i32>>, a: i32) {
    for j in 0..m.len() {
        for i in 0..m[0].len() {
            m[j][i] *= a;
        }
    }
}

pub fn submatrix<T: num::Num + Copy>(pos: (usize, usize), m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut c: Vec<Vec<T>> = Vec::new();

    for i in 0..m[0].len() {
        if i == pos.1 {
            continue;
        }

        c.push(Vec::new());
        let len = c.len() - 1;

        for j in 0..m.len() {
            if j == pos.0 {
                continue;
            } else {
                c[len].push(m[i][j]);
            }
        }
    }

    return c;
}

pub fn cofactor<T: num::Signed + Copy>(pos: (usize, usize), v: Vec<Vec<T>>) -> T {
    let submatrix = submatrix(pos, v);

    let det = laplace(&submatrix);

    // If i + j is odd
    if (pos.0 + pos.1) & 0b1 == 1 {
        return -det;
    } else {
        return det;
    }
}

pub fn matrix_inverse<T: num::Signed + Copy + num::ToPrimitive, U: num::Float>(
    v: &Vec<Vec<T>>,
) -> Vec<Vec<U>> {
    let mut b: Vec<Vec<U>> = Vec::new();
    let det = laplace(v);

    let ilen = v.len();
    let jlen = v[0].len();

    for i in 0..ilen {
        b.push(Vec::new());
        for j in 0..jlen {
            let cof = cofactor((i, j), v.to_vec());
            b[i].push(-U::from(cof).unwrap() / -U::from(det).unwrap());
        }
    }

    return b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_inverse() {
        assert_eq!(
            matrix_inverse::<i64, f64>(&vec![vec![2, 4, 3], vec![6, 1, 5], vec![-2, 1, 3]]),
            vec![
                vec![1.0 / 46.0, 9.0 / 92.0, -17.0 / 92.0],
                vec![7.0 / 23.0, -3.0 / 23.0, -2.0 / 23.0],
                vec![-2.0 / 23.0, 5.0 / 46.0, 11.0 / 46.0]
            ]
        )
    }

    #[test]
    fn test_matrix_multiply_equal_size() {
        assert_eq!(
            multiply(
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]
            ),
            [[84, 90, 96], [201, 216, 231], [318, 342, 366]]
        );
    }

    #[test]
    fn test_matrix_multiply_different_sizes() {
        assert_eq!(
            multiply(
                vec![vec![6, 24, 1], vec![13, 16, 10], vec![20, 17, 15]],
                vec![vec![0], vec![2], vec![19]]
            ),
            vec![vec![67], vec![222], vec![319]]
        );
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let mut v = vec![vec![1, 2], vec![3, 4]];
        scalar_matrix_multiplication(&mut v, 10);
        assert_eq!(v, vec![vec![10, 20], vec![30, 40]]);
    }

    #[test]
    fn test_submatrix() {
        assert_eq!(
            submatrix((1, 0), vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![4, 6], vec![7, 9]]
        )
    }

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

    #[test]
    fn test_cofactor() {
        assert_eq!(
            cofactor((2, 1), vec![vec![2, 4, 3], vec![6, 1, 5], vec![-2, 1, 3]]),
            -10
        );
    }
}
