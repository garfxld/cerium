use cerium_protocol_macros::packet;
use uuid::Uuid;

use crate::auth::{GameProfile, Property};
use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("login_finished", 0x02)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
}

impl Decode for LoginSuccessPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            uuid: r.read_uuid()?,
            username: r.read_string()?,
            properties: r.read_array(Property::decode)?,
        })
    }
}

impl Encode for LoginSuccessPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_uuid(this.uuid)?;
        w.write_string(this.username)?;
        w.write_array(this.properties, Property::encode)?;
        Ok(())
    }
}

impl From<GameProfile> for LoginSuccessPacket {
    fn from(value: GameProfile) -> Self {
        Self {
            uuid: value.uuid,
            username: value.name,
            properties: value.properties,
        }
    }
}
