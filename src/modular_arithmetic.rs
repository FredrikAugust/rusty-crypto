extern crate num;

fn is_even(n: i128) -> bool {
    return n % 2 == 0;
}

fn square_multiply_algorithm(n: i128, e: i128) -> i128 {
    if e == 0 {
        return 1;
    } else if e == 1 {
        return n;
    }

    match is_even(e) {
        false => return n * square_multiply_algorithm(n.pow(2), (e - 1) / 2),
        true => return square_multiply_algorithm(n.pow(2), e / 2),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_multiply_algorithm() {
        assert_eq!(square_multiply_algorithm(2, 126), 2i128.pow(126));
    }
}
