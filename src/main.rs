mod euclidean_algorithm;
mod euler;
mod kasiski_test;
mod matrix;
mod miller_rabin;

fn main() {
    println!("========== OUTPUT ==========");
    println!(
        "{:?}",
        matrix::dot(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]
        )
    )
}
