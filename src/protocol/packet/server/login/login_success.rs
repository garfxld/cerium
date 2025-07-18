use macros::packet;
use uuid::Uuid;

use crate::{
    network::auth::{GameProfile, Property},
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeException},
    },
};

#[derive(Debug, Clone)]
#[packet("login_finished")]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
}

impl Encode for LoginSuccessPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_uuid(this.uuid)?;
        buffer.write_string(this.username)?;

        buffer.write_array(this.properties, |buffer, value| {
            Property::encode(buffer, value)
        })?;
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
