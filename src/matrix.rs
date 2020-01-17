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

#[cfg(test)]
mod tests {
    use super::*;

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
}
