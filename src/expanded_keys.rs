use crate::{transform, Word, MASK, P, Q, WORD_SIZE_BYTES};
use anyhow::{bail, Result};

pub fn generate_expanded_keys_array<const S_SIZE: usize>(key: &[u8]) -> Result<[Word; S_SIZE]> {
    let key_with_words: Vec<Word> =
        transform::from_bytes_to_words::<WORD_SIZE_BYTES>(key).collect();

    let mut keys = init_expanded_keys_array();
    mix_expanded_keys_array(&mut keys, key_with_words)?;

    Ok(keys)
}

fn mix_expanded_keys_array(s: &mut [Word], key: Vec<Word>) -> Result<()> {
    let mut key = key;
    let key_size = key.len();
    let keys_extended_size = s.len();
    // min(2*(R+1)) = 2
    if keys_extended_size < 2 {
        bail!("the size of extended key table should be at least 2")
    }

    let mut i: usize = 0;
    let mut j: usize = 0;

    let mut word_a: Word = 0;
    let mut word_b: Word = 0;

    for _ in 0..(3 * usize::max(keys_extended_size, key_size)) {
        word_a = s[i]
            .wrapping_add(word_a)
            .wrapping_add(word_b)
            .rotate_left(3);
        s[i] = word_a;

        // According to spec its allowed to have key.len() == 0. Obviously, in this
        // situation we don't get any encryption
        if key_size > 0 {
            word_b = key[j]
                .wrapping_add(word_a)
                .wrapping_add(word_b)
                .rotate_left(word_a.wrapping_add(word_b) & MASK);
            key[j] = word_b;
            j = (j + 1) % key_size;
        }

        i = (i + 1) % keys_extended_size;
    }

    Ok(())
}

// Returns the the initialized expanded keys array
// Construction of array follows the pattern: S[n] = S[n-1] + Q
const fn init_expanded_keys_array<const S_SIZE: usize>() -> [Word; S_SIZE] {
    let mut expanded_keys: [Word; S_SIZE] = [0; S_SIZE];
    expanded_keys[0] = P;

    let mut i = 1;
    while i < S_SIZE {
        expanded_keys[i] = expanded_keys[i - 1].wrapping_add(Q);
        i += 1;
    }
    expanded_keys
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore = "should be implemented"]
    fn test_generate_expanded_keys() {}

    #[test]
    #[ignore = "should be implemented"]
    fn test_mix_expanded_keys_key_size_equals_0() {}

    #[test]
    #[ignore = "should be implemented"]
    fn test_mix_expanded_keys_size_equals_0() {}

    #[test]
    #[ignore = "should be implemented"]
    fn test_mix_expanded_keys_key_greater_than_0() {}
}
