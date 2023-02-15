use crate::params::{Word, BLOCK_SIZE, MASK};

/// Decrypts a one block. Please be aware this function may panic. Error checking is
/// is disabled because of performance reasons. The safety checks should be done outside the function.
#[allow(clippy::unnecessary_cast)]
#[inline]
pub fn decrypt_block<const S_SIZE: usize>(
    rounds: usize,
    expanded_keys: &[Word; S_SIZE],
    ciphertext: &[Word; BLOCK_SIZE],
    plaintext: &mut [Word; BLOCK_SIZE],
) {
    let mut word_a = ciphertext[0];
    let mut word_b = ciphertext[1];

    for i in (1..=rounds).rev() {
        word_b = word_b
            .wrapping_sub(expanded_keys[2 * i + 1])
            .rotate_right((word_a & MASK) as u32)
            ^ word_a;
        word_a = word_a
            .wrapping_sub(expanded_keys[2 * i])
            .rotate_right((word_b & MASK) as u32)
            ^ word_b;
    }

    plaintext[1] = word_b.wrapping_sub(expanded_keys[1]);
    plaintext[0] = word_a.wrapping_sub(expanded_keys[0]);
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
