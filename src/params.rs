/// The word size
pub type Word = u32;

// Number of rounds. The number of rounds should be between 0;255
pub const ROUNDS: usize = 12;
// for u16: 0xB7E1; for u64: 0xB7E1_5162_8AED_2A6B
pub const P: Word = 0xB7E1_5163;
// for u16: 0x9E37; for u64: 0x9E37_79B9_7F4A_7C15
pub const Q: Word = 0x9E37_79B9;

/// Number of bytes in one Word
pub const WORD_SIZE_BYTES: usize = std::mem::size_of::<Word>();
/// Number of words in each block. The size of block cannot be changed.
pub const BLOCK_SIZE: usize = 2;
/// Mask is used to obtain the number of positions to rotate
pub const MASK: Word = WORD_SIZE_BYTES as Word * 8 - 1;
