mod euclidean_algorithm;
use euclidean_algorithm::*;

fn main() {
    println!("{}", euclidean_algorithm(7121842, 817346138));
    println!("{:?}", extended_euclidean_algorithm(7121842, 817346138));
}
