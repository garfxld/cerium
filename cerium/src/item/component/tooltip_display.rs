use crate::{
    item::AnyDataComponent,
    protocol::{
        DataType,
        decode::{Decode as _, DecodeError, PacketRead},
        encode::{Encode as _, EncodeError, PacketWrite},
    },
};

#[derive(Debug, Clone)]
pub struct TooltipDisplay {
    hide_toolip: bool,
    hidden_components: Vec<AnyDataComponent>,
}

impl DataType for TooltipDisplay {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            hide_toolip: r.read_bool()?,
            hidden_components: r.read_array(|r| AnyDataComponent::decode(r))?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_bool(this.hide_toolip)?;
        w.write_array(&this.hidden_components, |w, v| {
            AnyDataComponent::encode(w, v)
        })?;
        Ok(())
    }
}
