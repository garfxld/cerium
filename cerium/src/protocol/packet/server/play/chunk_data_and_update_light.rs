use crate::world::{BlockEntity, heightmap::Heightmap};
use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("level_chunk_with_light", 0x27)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: ChunkData,
    pub light: LightData,
}

impl Decode for ChunkDataAndUpdateLightPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            chunk_x: r.read_i32()?,
            chunk_z: r.read_i32()?,
            data:    ChunkData::decode(r)?,
            light:   LightData::decode(r)?,
        })
    }
}

impl Encode for ChunkDataAndUpdateLightPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i32(this.chunk_x)?;
        w.write_i32(this.chunk_z)?;
        ChunkData::encode(w, this.data)?;
        LightData::encode(w, this.light)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: Vec<Heightmap>,
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntity>,
}

impl Decode for ChunkData {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            heightmaps:     r.read_array(Heightmap::decode)?,
            data:           r.read_array(|r| r.read_u8())?,
            block_entities: r.read_array(BlockEntity::decode)?,
        })
    }
}

impl Encode for ChunkData {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_array(this.heightmaps, Heightmap::encode)?;
        w.write_array(this.data, |b, v| b.write_u8(v))?;
        w.write_array(this.block_entities, BlockEntity::encode);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LightData {}

impl Decode for LightData {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for LightData {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        let num_sections = 26;

        w.write_varint(1)?;
        w.write_u64(0x3FFFFFF_u64)?;

        w.write_varint(0)?;

        w.write_varint(0)?;
        w.write_varint(1)?;
        w.write_u64(0x3FFFFFF_u64)?;

        let light_array = vec![0xFF; 2048];
        w.write_varint(num_sections as i32)?;
        for _ in 0..num_sections {
            w.write_array(light_array.clone(), |w, v| w.write_u8(v));
        }
        w.write_varint(0)?;

        Ok(())
    }
}
