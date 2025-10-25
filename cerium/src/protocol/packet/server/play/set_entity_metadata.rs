use rustc_hash::FxHashMap;

use crate::{
    entity::meta::AnyValue,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};

#[derive(Debug, Clone)]
pub struct SetEntityMetadataPacket {
    pub entity_id: i32,
    pub entries: FxHashMap<i32, AnyValue>,
}

impl Packet for SetEntityMetadataPacket {}
impl ServerPacket for SetEntityMetadataPacket {}

impl Encode for SetEntityMetadataPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;

        for (key, value) in &this.entries {
            w.write_varint(*key)?; // index
            AnyValue::encode_value(value, w)?; // value (type + data)
        }
        w.write_u8(0xFF)?;
        Ok(())
    }
}

impl Decode for SetEntityMetadataPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let entity_id = r.read_varint()?;

        let mut entries = FxHashMap::default();
        loop {
            let index = r.read_varint()?;
            if index == 0xFF {
                break;
            }

            let value = AnyValue::decode_value(r)?;
            entries.insert(index, value);
        }

        Ok(Self { entity_id, entries })
    }
}
