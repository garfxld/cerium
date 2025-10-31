use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::TextComponent,
};

#[derive(Debug, Clone)]
pub struct UpdateObjectivesPacket {
    pub objective_name: String,
    pub mode: i8,
    pub objective_value: Option<TextComponent>,
    pub ty: Option<i32>,
    pub has_number_format: Option<bool>,
    pub number_format: Option<i32>,
}

impl Packet for UpdateObjectivesPacket {}
impl ServerPacket for UpdateObjectivesPacket {}

impl Encode for UpdateObjectivesPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.objective_name)?;
        w.write_i8(this.mode)?;

        if !matches!(this.mode, 0 | 2) {
            return Ok(());
        }

        w.write_component(this.objective_value.as_ref().unwrap())?;
        w.write_varint(this.ty.unwrap())?;
        w.write_bool(this.has_number_format.unwrap())?;
        if this.has_number_format.unwrap() {
            w.write_varint(this.number_format.unwrap())?;
        }
        Ok(())
    }
}
