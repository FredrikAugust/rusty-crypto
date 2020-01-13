mod euclidean_algorithm;
mod kasiski_test;

fn main() {
    println!("{}", euclidean_algorithm::euclidean_algorithm(24, 36));
    println!("{:?}", euclidean_algorithm::bezout_coefficients(24, 36));
    println!(
        "{}",
        kasiski_test::kasiski_test("abcdefghijklmabcdefklikklmabcdefa", 6).unwrap()
    );
}
