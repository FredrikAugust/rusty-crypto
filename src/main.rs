mod euclidean_algorithm;
mod euler;
mod kasiski_test;
mod miller_rabin;

fn main() {
    println!("{}", euclidean_algorithm::euclidean_algorithm(24, 36));
    println!("{:?}", euclidean_algorithm::bezout_coefficients(24, 36));
    println!(
        "{}",
        kasiski_test::kasiski_test("abcdefghijklmabcdefklikklmabcdefa", 6).unwrap()
    );
    println!("{}", euler::euler_totient(37));
    println!("{}", euler::euler_totient(35));
    println!("{}", miller_rabin::miller_rabin_test(29));
}
