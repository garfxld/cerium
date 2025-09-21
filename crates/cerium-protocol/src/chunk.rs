use cerium_world::{
    chunk::{BlockEntity, Chunk},
    chunk_section::ChunkSection,
    heightmap::Heightmap,
    palette::{Palette, PaletteFormat},
};

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
    packet::{ChunkData, ChunkDataAndUpdateLightPacket, LightData},
};

impl Encode for BlockEntity {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_u8(this.packed_xz)?;
        buffer.write_i16(this.y)?;
        buffer.write_varint(this.r#type)?;

        let mut data: Vec<u8> = Vec::new();
        this.data.write_unnamed(&mut data);
        buffer.put(&*data);
        Ok(())
    }
}

impl Encode for Heightmap {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.kind)?;
        buffer.write_array(this.data, |buffer, value| buffer.write_i64(value))?;
        Ok(())
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

impl Encode for ChunkSection {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_i16(this.block_states.count() as i16)?;
        Palette::encode(buffer, this.block_states)?;
        Palette::encode(buffer, this.biomes)?;
        Ok(())
    }
}

impl Into<ChunkDataAndUpdateLightPacket> for &Chunk {
    fn into(self) -> ChunkDataAndUpdateLightPacket {
        let mut data = ByteBuffer::new();
        for section in self.sections() {
            ChunkSection::encode(&mut data, section.clone()).unwrap();
        }

        let chunk_x = self.x();
        let chunk_z = self.z();

        let data = ChunkData {
            heightmaps: vec![],
            data: data.to_vec(),
            block_entities: self.block_entites().into_iter().cloned().collect(),
        };
        let light = LightData {};

        ChunkDataAndUpdateLightPacket {
            chunk_x,
            chunk_z,
            data,
            light,
        }
    }
}
