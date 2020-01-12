pub fn euclidean_algorithm(a: u32, b: u32) -> u32 {
    let mut r: Vec<u32> = vec![a, b];
    let mut k = 1;

    while r[k] != 0 {
        let q = (r[k - 1] / r[k]) as f32;
        let q = q.floor() as u32;
        r.push(r[k - 1] - q * r[k]);
        k += 1;
    }

    k -= 1;

    return r[k];
}
