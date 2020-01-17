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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_dot_test() {
        assert_eq!(
            dot(
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]
            ),
            [[84, 90, 96], [201, 216, 231], [318, 342, 366]]
        );
    }
}
