mod euclidean_algorithm;
mod euler;
mod kasiski_test;
mod miller_rabin;

fn main() {
    println!("========== OUTPUT ==========");
    println!("{}", euclidean_algorithm::find_modular_inverse(3, 31));
    println!("{}", euclidean_algorithm::find_modular_inverse(21, 91));
    println!("{}", euclidean_algorithm::find_modular_inverse(39, 195));
    println!("{}", euclidean_algorithm::find_modular_inverse(41, 195));
}
