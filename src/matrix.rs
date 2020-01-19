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

pub fn dot(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut c: Vec<Vec<i32>> = vec![];

    for i in 0..a[0].len() {
        c.push(Vec::new());

        for j in 0..b.len() {
            let mut acc = 0i32;

            for k in 0..b[0].len() {
                acc += &a[i][k] * &b[k][j];
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

pub fn submatrix(pos: (usize, usize), m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut c: Vec<Vec<i32>> = Vec::new();

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

pub fn cofactor(pos: (usize, usize), v: Vec<Vec<i32>>) -> i32 {
    let submatrix = submatrix(pos, v);

    let det = laplace(&submatrix);

    // If i + j is odd
    if (pos.0 + pos.1) & 0b1 == 1 {
        return -det;
    } else {
        return det;
    }
}

pub fn matrix_inverse(v: &Vec<Vec<i32>>) -> Vec<Vec<f64>> {
    let mut b: Vec<Vec<f64>> = Vec::new();
    let det = laplace(&v) as f64;

    let ilen = v.len();
    let jlen = v[0].len();

    for i in 0..ilen {
        b.push(Vec::new());
        for j in 0..jlen {
            let cof = cofactor((i, j), v.to_vec()) as f64;
            b[i].push(-cof / -det);
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
            matrix_inverse(&vec![vec![2, 4, 3], vec![6, 1, 5], vec![-2, 1, 3]]),
            vec![
                vec![1.0 / 46.0, 9.0 / 92.0, -17.0 / 92.0],
                vec![7.0 / 23.0, -3.0 / 23.0, -2.0 / 23.0],
                vec![-2.0 / 23.0, 5.0 / 46.0, 11.0 / 46.0]
            ] as Vec<Vec<f64>>
        )
    }

    #[test]
    fn test_matrix_dot() {
        assert_eq!(
            dot(
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]
            ),
            [[84, 90, 96], [201, 216, 231], [318, 342, 366]]
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
