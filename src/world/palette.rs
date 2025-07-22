use std::collections::{hash_map::Entry, HashMap};

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug, Clone)]
pub enum PaletteFormat {
    SingleValued { value: i32 },
    Indirect { values: Vec<i32> },
    Direct,
}

#[derive(Debug, Clone)]
pub struct Palette {
    dim: usize,
    min_bpe: u8,
    max_bpe: u8,
    direct_bpe: u8,

    bpe: u8,
    format: PaletteFormat,
    values: Vec<i64>,

    data: Box<[u16]>,
    count: HashMap<u16, i32>,
}

impl Palette {
    pub fn blocks() -> Self {
        Palette::empty(16, 4, 8, 15)
    }

    pub fn biomes() -> Self {
        Palette::empty(4, 1, 3, 6)
    }

    fn empty(dim: usize, min_bpe: u8, max_bpe: u8, direct_bpe: u8) -> Self {
        let mut count = HashMap::with_capacity(1);
        count.insert(0, (dim * dim * dim) as i32);

        Self {
            dim,
            min_bpe,
            max_bpe,
            direct_bpe,

            bpe: 0,
            format: PaletteFormat::SingleValued { value: 0 },
            values: vec![],

            data: vec![0u16; dim * dim * dim].into_boxed_slice(),
            count,
        }
    }

    #[inline]
    const fn index(&self, x: usize, y: usize, z: usize) -> usize {
        y * self.dim * self.dim + z * self.dim + x
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u16 {
        if x >= self.dim || y >= self.dim || z >= self.dim {
            panic!("Index out of bounds: ({}, {}, {})", x, y, z);
        }

        self.data[self.index(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: u16) {
        let index = self.index(x, y, z);
        let original = self.data[index];

        if let Entry::Occupied(mut entry) = self.count.entry(original) {
            let count = entry.get_mut();
            *count -= 1;
            if *count == 0 {
                let _ = entry.remove();
            }
        }

        self.data[index] = value;
        self.count
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    /// Returns the number of non-air blocks in the palette.
    pub fn count(&self) -> i32 {
        self.count
            .iter()
            .map(|(k, v)| if *k == 0 { 0 } else { *v }) // todo: add cave- and void air
            .sum()
    }

    /// Calculates the required bits per entry for the given number of block states.
    #[inline]
    const fn required_bpe(block_states: i32) -> u32 {
        if block_states <= 1 {
            0
        } else {
            32 - (block_states - 1).leading_zeros()
        }
    }

    fn compute(&mut self) {
        let bpe = Palette::required_bpe(self.count.len() as i32) as u8;
        if bpe == 0 {
            // SingleValued Format
            self.bpe = 0;
            self.format = PaletteFormat::SingleValued {
                value: *self.count.keys().next().unwrap_or(&0) as i32,
            };
            self.values = vec![];
            return;
        }

        if bpe <= self.max_bpe {
            // Indirect Format

            let bpe = bpe.max(self.min_bpe) as u8;

            let palette: Box<[u16]> = self
                .count
                .keys()
                .copied()
                .collect::<Vec<u16>>()
                .into_boxed_slice();
            let key_to_index_map: HashMap<u16, usize> = palette
                .iter()
                .enumerate()
                .map(|(index, key)| (*key, index))
                .collect();

            let flat_values = &self.data;
            let values: Vec<i64> = flat_values
                .chunks(64 / bpe as usize)
                .map(|chunk| {
                    let mut packed_long = 0u64;

                    for (entry_index, &block_id) in chunk.iter().enumerate() {
                        let palette_index = *key_to_index_map
                            .get(&block_id)
                            .expect("Block ID not found in palette");

                        debug_assert!(palette_index < (1 << bpe));
                        let bit_offset = entry_index * bpe as usize;

                        packed_long |= (palette_index as u64) << bit_offset;
                    }

                    packed_long as i64
                })
                .collect();

            self.bpe = bpe;
            self.format = PaletteFormat::Indirect {
                values: palette.iter().map(|v| *v as i32).collect(),
            };
            self.values = values;
        } else {
            // Direct Format
            let bpe = self.direct_bpe;

            let values: Vec<i64> = self
                .data
                .chunks(64 / bpe as usize)
                .map(|chunk| {
                    chunk.iter().enumerate().fold(0, |acc, (index, value)| {
                        debug_assert!((1 << bpe) > *value);

                        let packed_offset_index = (*value as i64) << (bpe as u64 * index as u64);
                        acc | packed_offset_index
                    })
                })
                .collect();

            self.bpe = bpe;
            self.format = PaletteFormat::Direct;
            self.values = values;
        }
    }
}

impl Encode for Palette {
    fn encode(buffer: &mut ByteBuffer, mut this: Self) -> Result<(), EncodeError> {
        this.compute();

        buffer.write_u8(this.bpe)?;
        PaletteFormat::encode(buffer, this.format)?;
        for value in this.values {
            buffer.write_i64(value)?;
        }

        Ok(())
    }
}

impl Encode for PaletteFormat {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        match this {
            PaletteFormat::SingleValued { value } => {
                buffer.write_varint(value)?;
            }
            PaletteFormat::Indirect { values } => {
                buffer.write_array(values, |buffer, value| buffer.write_varint(value))?;
            }
            PaletteFormat::Direct => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpe() {
        assert_eq!(Palette::required_bpe(1), 0);
        assert_eq!(Palette::required_bpe(2), 1);
        assert_eq!(Palette::required_bpe(3), 2);
        assert_eq!(Palette::required_bpe(4), 2);
        assert_eq!(Palette::required_bpe(5), 3);
        assert_eq!(Palette::required_bpe(10), 4);
        assert_eq!(Palette::required_bpe(100), 7);
        assert_eq!(Palette::required_bpe(500), 9);
        assert_eq!(Palette::required_bpe(1000), 10);
    }

    #[test]
    fn test_palette_count() {
        let mut palette = Palette::blocks();
        assert_eq!(palette.count(), 0);

        palette.set(0, 0, 0, 1);
        palette.set(0, 0, 1, 2);
        assert_eq!(palette.count(), 2);

        palette.set(0, 0, 0, 3);
        assert_eq!(palette.count(), 2);

        palette.set(0, 0, 0, 0);
        assert_eq!(palette.count(), 1);
    }
}
