use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: Box<[u8]>,
    pub verify_token: Box<[u8]>,
    pub should_authenticate: bool,
}

impl Packet for EncryptionRequestPacket {}
impl ServerPacket for EncryptionRequestPacket {}

impl Encode for EncryptionRequestPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.server_id)?;
        w.write_boxed_slice(&this.public_key)?;
        w.write_boxed_slice(&this.verify_token)?;
        w.write_bool(this.should_authenticate)?;
        Ok(())
    }
}
