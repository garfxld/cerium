use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::TextComponent,
};

#[derive(Debug, Clone)]
pub struct UpdateScorePacket {
    pub entity_name: String,
    pub objective_name: String,
    pub value: i32,
    pub display_name: Option<TextComponent>,
    pub number_format: Option<i32>,
}

impl Packet for UpdateScorePacket {}
impl ServerPacket for UpdateScorePacket {}

impl Encode for UpdateScorePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.entity_name)?;
        w.write_string(&this.objective_name)?;
        w.write_varint(this.value)?;
        w.write_option(&this.display_name, |w, v| w.write_component(v))?;
        w.write_option(&this.number_format, |w, v| w.write_varint(*v))?;
        Ok(())
    }
}
