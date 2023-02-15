use crate::params::{Word, BLOCK_SIZE, MASK};

/// Encrypts a one block. Please be aware this function may panic. Error checking is
/// is disabled because of performance reasons. The safety checks should be done outside the function.
#[allow(clippy::unnecessary_cast)]
#[inline]
pub fn encrypt_block<const S_SIZE: usize>(
    rounds: usize,
    expanded_keys: &[Word; S_SIZE],
    plaintext: &[Word; BLOCK_SIZE],
    ciphertext: &mut [Word; BLOCK_SIZE],
) {
    let mut word_a = plaintext[0].wrapping_add(expanded_keys[0]);
    let mut word_b = plaintext[1].wrapping_add(expanded_keys[1]);

    for i in 1..=rounds {
        word_a = (word_a ^ word_b)
            .rotate_left((word_b & MASK) as u32)
            .wrapping_add(expanded_keys[2 * i]);
        word_b = (word_b ^ word_a)
            .rotate_left((word_a & MASK) as u32)
            .wrapping_add(expanded_keys[2 * i + 1]);
    }

    ciphertext[0] = word_a;
    ciphertext[1] = word_b;
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore = "should be implemented"]
    fn decrypt_block_with_1_round_3_size_expanded_key() {}

    #[test]
    #[ignore = "should be implemented"]
    fn decrypt_block_with_0_round() {}
}
