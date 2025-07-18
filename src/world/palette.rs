use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug, Clone)]
pub enum PaletteFormat {
    SingleValued { value: i32 },
    Indirect { values: Vec<i32> },
    Direct,
}

#[derive(Debug, Clone)]
pub struct Palette {
    dimension: i32,
    min_bpe: i32,
    max_bpe: i32,
    direct_bpe: i32,

    bpe: u8,
    format: PaletteFormat,
    data: Vec<i64>,
}

impl Palette {
    pub fn blocks() -> Self {
        Self::empty(16, 4, 8, 15)
    }

    pub fn biomes() -> Self {
        Self::empty(4, 1, 3, 6)
    }

    fn empty(dimension: i32, min_bpe: i32, max_bpe: i32, direct_bpe: i32) -> Self {
        Self {
            dimension,
            min_bpe,
            max_bpe,
            direct_bpe,

            bpe: 0,
            format: PaletteFormat::SingleValued { value: 9.into() },
            data: vec![],
        }
    }

    pub fn set(&self, x: i32, y: i32, z: i32, block: i32) {}

    pub fn count(&self) -> i32 {
        match &self.format {
            PaletteFormat::SingleValued { value } => {
                if [0].contains(&value) {
                    0
                } else {
                    self.dimension * self.dimension * self.dimension
                }
            }
            _ => todo!(),
        }
    }
}

impl Encode for Palette {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_u8(this.bpe)?;

        match this.format {
            PaletteFormat::SingleValued { value } => {
                buffer.write_varint(value)?;
            }
            PaletteFormat::Indirect { values } => {
                let _ = values;
                for value in this.data {
                    buffer.write_i64(value)?;
                }
            }
            PaletteFormat::Direct => {
                for value in this.data {
                    buffer.write_i64(value)?;
                }
            }
        }
        Ok(())
    }
}
