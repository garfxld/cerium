use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Identifier,
};

#[derive(Debug, Clone)]
pub struct FeatureFlagsPacket {
    pub feature_flags: Vec<Identifier>,
}

impl Packet for FeatureFlagsPacket {}
impl ServerPacket for FeatureFlagsPacket {}

impl Encode for FeatureFlagsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.feature_flags, |w, v| w.write_identifier(v))?;
        Ok(())
    }
}
