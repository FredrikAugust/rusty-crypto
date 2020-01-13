mod euclidean_algorithm;
use euclidean_algorithm::*;

fn main() {
    println!("{}", euclidean_algorithm(24, 36));
    println!("{:?}", bezout_coefficients(24, 36));
}
