use bytes::BytesMut;
use cerium_world::{
    chunk::{BlockEntity, Chunk},
    chunk_section::ChunkSection,
    heightmap::Heightmap,
    palette::{Palette, PaletteFormat},
};

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::{ChunkData, ChunkDataAndUpdateLightPacket, LightData},
    read::PacketRead,
    write::PacketWrite,
};

impl Decode for BlockEntity {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            packed_xz: r.read_u8()?,
            y: r.read_i16()?,
            r#type: r.read_varint()?,
            data: r.read_nbt()?,
        })
    }
}

impl Encode for BlockEntity {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_u8(this.packed_xz)?;
        w.write_i16(this.y)?;
        w.write_varint(this.r#type)?;
        w.write_nbt(this.data)?;
        Ok(())
    }
}

impl Decode for Heightmap {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: r.read_varint()?,
            data: r.read_array(|r| r.read_i64())?,
        })
    }
}

impl Encode for Heightmap {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.kind)?;
        w.write_array(this.data, |w, v| w.write_i64(v))?;
        Ok(())
    }
}

impl Encode for Palette {
    fn encode<W: PacketWrite>(w: &mut W, mut this: Self) -> Result<(), EncodeError> {
        this.compute();

        w.write_u8(this.bpe)?;
        PaletteFormat::encode(w, this.format)?;
        for value in this.values {
            w.write_i64(value)?;
        }

        Ok(())
    }
}

impl Encode for PaletteFormat {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        match this {
            PaletteFormat::SingleValued { value } => {
                w.write_varint(value)?;
            }
            PaletteFormat::Indirect { values } => {
                w.write_array(values, |buffer, value| buffer.write_varint(value))?;
            }
            PaletteFormat::Direct => {}
        }
        Ok(())
    }
}

impl Encode for ChunkSection {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i16(this.block_states.count() as i16)?;
        Palette::encode(w, this.block_states)?;
        Palette::encode(w, this.biomes)?;
        Ok(())
    }
}

impl Into<ChunkDataAndUpdateLightPacket> for &Chunk {
    fn into(self) -> ChunkDataAndUpdateLightPacket {
        let mut data = BytesMut::new();
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
