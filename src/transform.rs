use crate::params::Word;

// Returns an iterator transforming bytes into the n-sized words.
// If the len(bytes) % word_size != 0, then rest is padded with zeros
pub fn from_bytes_to_words<const WORD_SIZE_BYTES: usize>(
    bytes: &[u8],
) -> impl Iterator<Item = Word> + '_ {
    bytes.chunks(WORD_SIZE_BYTES).map(|chunk| {
        let mut word: Word = 0;
        for b in chunk {
            let r = b.to_owned() as Word;
            word = (word.rotate_left(8)) | r;
        }
        word.swap_bytes()
    })
}

// Returns an iterator transforming words into the bytes
pub fn from_words_to_bytes<const WORD_SIZE_BYTES: usize>(
    words: &[Word],
) -> impl Iterator<Item = u8> + '_ {
    words.iter().flat_map(|word| {
        let mut bytes = [0u8; WORD_SIZE_BYTES];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = ((word.rotate_right((i * 8) as u32)) & 0xff) as u8;
        }
        bytes
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_bytes_to_words() {
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
        let expect: Vec<Word> = vec![0x78_56_34_12];
        let words: Vec<Word> = from_bytes_to_words::<4>(&bytes).collect();
        assert_eq!(words, expect);
    }

    #[test]
    fn test_from_bytes_to_words_with_zer_padding() {
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56];
        let expect: Vec<Word> = vec![0x56_34_12_00];
        let words: Vec<Word> = from_bytes_to_words::<4>(&bytes).collect();
        assert_eq!(words, expect);
    }

    #[test]
    fn test_from_words_to_bytes() {
        let words: Vec<Word> = vec![0x12_34_56_78];
        let expect: Vec<u8> = vec![0x78, 0x56, 0x34, 0x12];
        let bytes: Vec<u8> = from_words_to_bytes::<4>(&words).collect();
        assert_eq!(bytes, expect);
    }
}
