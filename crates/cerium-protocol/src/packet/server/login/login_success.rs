use cerium_protocol_macros::packet;
use cerium_util::auth::{GameProfile, Property};
use uuid::Uuid;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug, Clone)]
#[packet("login_finished")]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
}

impl Encode for LoginSuccessPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
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
