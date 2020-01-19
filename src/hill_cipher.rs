#[path = "matrix.rs"]
mod matrix;

fn char_to_code(a: char) -> u32 {
    // A -> 0, Z -> 25
    return (a as u32) - 0x41;
}

fn code_to_char(c: u32) -> char {
    return ((c + 0x41) as u8) as char;
}

pub fn hill_cipher(text: &str, key: Vec<Vec<u32>>) -> String {
    if text.len() % key.len() != 0 {
        panic!("Text length must be a multiple of y-dimension of key.");
    }

    // Create matrix of text
    let mut text_matrix = Vec::new();

    // Create rows
    for _ in 0..key.len() {
        text_matrix.push(Vec::<u32>::new());
    }

    for (i, c) in text.chars().enumerate() {
        text_matrix[i % key.len()].push(char_to_code(c).into());
    }

    let mut ciphertext = matrix::multiply(key, text_matrix);

    matrix::modulus(&mut ciphertext, 26);

    let mut result = String::new();

    for i in 0..ciphertext[0].len() {
        for j in 0..ciphertext.len() {
            result.push(code_to_char(ciphertext[j][i]));
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hill_cipher_encryption() {
        assert_eq!(
            hill_cipher(
                "ACT",
                vec![vec![6, 24, 1], vec![13, 16, 10], vec![20, 17, 15]]
            ),
            "POH"
        );
    }

    #[test]
    fn test_char_to_code() {
        assert_eq!(char_to_code('A'), 0);
        assert_eq!(char_to_code('Z'), 25);
    }

    #[test]
    fn test_code_to_char() {
        assert_eq!(code_to_char(0), 'A');
        assert_eq!(code_to_char(25), 'Z');
    }
}