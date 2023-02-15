use anyhow::{bail, Result};
use log::warn;

use crate::expanded_keys::generate_expanded_keys_array;
use crate::params::*;
use crate::{decrypt::decrypt_block, encrypt::encrypt_block, transform};

/// The size of expansion key table
const S_SIZE: usize = 2 * (ROUNDS + 1);

/// The implementation of RC-32/12/b
#[derive(Debug, Clone)]
pub struct RC5 {
    expanded_keys: [Word; S_SIZE],
}

impl RC5 {
    pub fn new(key: &[u8]) -> Result<Self, anyhow::Error> {
        if key.len() > 255 {
            bail!("The max size of key is 255");
        }
        if key.is_empty() {
            warn!("The length of the key is 0. No encryption is provided")
        }

        Ok(Self {
            expanded_keys: generate_expanded_keys_array(key)?,
        })
    }

    /// Implements an naive ECB block cipher mode for encryption.
    /// *Please be aware, this way o applying blocks isn't secure
    pub fn encrypt_with_ecb(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut plaintext_by_words = transform::from_bytes_to_words::<WORD_SIZE_BYTES>(plaintext);
        let mut ciphertext: Vec<u8> = vec![];

        // the implementation should be changed when array_chunks() get stabilized
        loop {
            let word_a = plaintext_by_words.next();
            if word_a.is_none() {
                break;
            }
            let word_b = plaintext_by_words.next().unwrap_or_default();

            if word_a.is_none() {
                break;
            }

            let mut output_block: [Word; 2] = [0; 2];
            self.encrypt_block(&[word_a.unwrap(), word_b], &mut output_block);

            let bytes = transform::from_words_to_bytes::<WORD_SIZE_BYTES>(&output_block);
            ciphertext.extend(bytes)
        }

        ciphertext
    }

    /// Implements an naive ECB block cipher mode for decryption.
    /// *Please be aware, this way o applying blocks isn't secure
    pub fn decrypt_with_ecb(&self, ciphertext: &[u8]) -> Vec<u8> {
        let mut ciphertext_by_words = transform::from_bytes_to_words::<WORD_SIZE_BYTES>(ciphertext);
        let mut plaintext: Vec<u8> = vec![];

        // the implementation should be changed when array_chunks() get stabilized
        loop {
            let word_a = ciphertext_by_words.next();
            if word_a.is_none() {
                break;
            }
            let word_b = ciphertext_by_words.next().unwrap_or_default();

            let mut output_block: [Word; 2] = [0; 2];
            self.decrypt_block(&[word_a.unwrap(), word_b], &mut output_block);

            let bytes = transform::from_words_to_bytes::<WORD_SIZE_BYTES>(&output_block);
            plaintext.extend(bytes)
        }

        plaintext
    }

    pub fn encrypt_block(
        &self,
        plaintext: &[Word; BLOCK_SIZE],
        ciphertext: &mut [Word; BLOCK_SIZE],
    ) {
        encrypt_block(ROUNDS, &self.expanded_keys, plaintext, ciphertext)
    }

    pub fn decrypt_block(
        &self,
        ciphertext: &[Word; BLOCK_SIZE],
        plaintext: &mut [Word; BLOCK_SIZE],
    ) {
        decrypt_block(ROUNDS, &self.expanded_keys, ciphertext, plaintext)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn init() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }

    #[test]
    fn test_key_too_long() {
        let rc = RC5::new(&[0_u8; 256]).expect_err("error should be returned");

        assert!(rc.to_string().contains("The max size of key is 255"));
    }

    #[test]
    fn encrypt_0_payload() {
        let rc = RC5::new(&[0_u8]).expect("no error");
        let output = rc.encrypt_with_ecb(&[]);

        assert!(output.is_empty())
    }

    #[test]
    fn encode_a() {
        let key = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let pt = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let ct = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];

        let rc = RC5::new(&key).expect("instance should be created");
        let res = rc.encrypt_with_ecb(&pt);
        assert_eq!(&ct[..], &res[..]);
    }

    #[test]
    fn encode_b() {
        let key = vec![
            0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81,
            0xFF, 0x48,
        ];
        let pt = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
        let ct = vec![0x11, 0xE4, 0x3B, 0x86, 0xD2, 0x31, 0xEA, 0x64];

        let rc = RC5::new(&key).expect("instance should be created");
        let res = rc.encrypt_with_ecb(&pt);
        assert_eq!(&ct[..], &res[..]);
    }

    #[test]
    fn decode_a() {
        let key = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let pt = vec![0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62];
        let ct = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];

        let rc = RC5::new(&key).expect("instance should be created");
        let res = rc.decrypt_with_ecb(&ct);
        assert_eq!(&pt[..], &res[..]);
    }

    #[test]
    fn decode_b() {
        let key = vec![
            0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81,
            0xFF, 0x48,
        ];
        let pt = vec![0x63, 0x8B, 0x3A, 0x5E, 0xF7, 0x2B, 0x66, 0x3F];
        let ct = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];

        let rc = RC5::new(&key).expect("instance should be created");
        let res = rc.decrypt_with_ecb(&ct);

        assert_eq!(&pt[..], &res[..]);
    }
}
