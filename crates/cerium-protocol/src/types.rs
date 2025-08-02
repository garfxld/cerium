use cerium_util::auth::Property;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

impl Encode for Property {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_string(this.name)?;
        buffer.write_string(this.value)?;
        buffer.write_optional(this.signature, |buffer, value| buffer.write_string(value))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    /// Create an empty BitSet
    pub fn new() -> Self {
        Self { words: Vec::new() }
    }

    /// Create a BitSet with enough capacity for at least `num_bits` bits
    pub fn with_capacity(num_bits: usize) -> Self {
        let num_words = Self::word_index(num_bits.saturating_sub(1)) + 1;
        Self {
            words: vec![0; num_words],
        }
    }

    /// Create a BitSet from an existing Vec<u64>
    pub fn from_words(words: Vec<u64>) -> Self {
        Self { words }
    }

    /// Return the inner Vec<u64>
    pub fn to_words(&self) -> Vec<u64> {
        self.words.clone()
    }

    /// Set bit at index `bit_index` to 1
    pub fn set(&mut self, bit_index: usize) {
        let word_idx = Self::word_index(bit_index);
        self.ensure_capacity(word_idx);
        self.words[word_idx] |= 1 << (bit_index % 64);
    }

    /// Clear bit at index `bit_index` (set to 0)
    pub fn clear(&mut self, bit_index: usize) {
        let word_idx = Self::word_index(bit_index);
        if word_idx < self.words.len() {
            self.words[word_idx] &= !(1 << (bit_index % 64));
        }
    }

    /// Check if bit at index `bit_index` is set
    pub fn is_set(&self, bit_index: usize) -> bool {
        let word_idx = Self::word_index(bit_index);
        if word_idx < self.words.len() {
            (self.words[word_idx] & (1 << (bit_index % 64))) != 0
        } else {
            false
        }
    }

    /// Helper: get the word index for a given bit index
    #[inline]
    fn word_index(bit_index: usize) -> usize {
        bit_index / 64
    }

    /// Ensure the vector can hold at least word_idx + 1 words
    #[inline]
    fn ensure_capacity(&mut self, word_idx: usize) {
        if self.words.len() <= word_idx {
            self.words.resize(word_idx + 1, 0);
        }
    }
}

impl Encode for BitSet {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        let words = this.to_words();
        buffer.write_varint(words.len() as i32)?;
        for word in words {
            buffer.write_u64(word)?;
        }
        Ok(())
    }
}
