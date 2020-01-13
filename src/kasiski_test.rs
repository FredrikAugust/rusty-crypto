use std::collections::HashMap;

// Attempt to deduce the interval size of ciphertext encrypted with
// polyalphabetic subsistution ciphers. Recommended use of this is actually to
// use multiple word lengths, and try to find common factors between the
// results.
pub fn kasiski_test(ciphertext: &str, word_length: usize) -> Result<usize, String> {
    // Doesn't work that well if you don't have a perfectly divisible
    // ciphertext.

    // Keep track of how often intervals occur
    let mut results: HashMap<usize, i32> = HashMap::new();

    // Loop through, but leave out the last chars so as not to get an index out
    // of bounds when looping later
    for (i, _c) in ciphertext[..ciphertext.len() - word_length + 1]
        .chars()
        .enumerate()
    {
        // What we will try to split on
        let word = &ciphertext[i..i + word_length];

        // Create the split words, we will use this to check if we have common
        // intervals between "words"
        let splits: Vec<&str> = ciphertext.split(word).collect();

        // This means that we found the pattern more than once
        if &splits.len() >= &3 {
            for (split_index, split) in splits.iter().enumerate() {
                for split2_index in split_index..splits.len() {
                    if split == &splits[split2_index] {
                        // We need to factor in the distance from the beginning
                        // of the word we split on.
                        let len = split.len() + word_length;

                        // Check if we have the length recorded from before, and
                        // if so, increment it by one. If not, set it to one.
                        if let Some(x) = results.get_mut(&len) {
                            *x += 1;
                        } else {
                            results.insert(len, 1);
                        }
                    }
                }
            }
        }
    }

    // Get the results into a vector so we can sort it by the frequency.
    let mut results: Vec<(&usize, &i32)> = results.iter().collect();
    results.sort_by_key(|x| x.1);

    if let Some(x) = results.last() {
        return Ok(*x.0);
    } else {
        return Err(format!(
            "Could not find any repeating patterns of length {}.",
            word_length
        ));
    }
}
