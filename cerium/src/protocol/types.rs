use crate::auth::Property;
use crate::inventory::Slot;
use crate::item::{AnyDataComponent, ComponentMap, DataComponent};
use crate::protocol::encode::PacketWrite;
use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    encode::{Encode, EncodeError},
};

impl Decode for Property {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            name: r.read_string()?,
            value: r.read_string()?,
            signature: r.read_option(|r| r.read_string())?,
        })
    }
}

impl Encode for Property {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.name)?;
        w.write_string(&this.value)?;
        w.write_option(&this.signature, |buffer, value| buffer.write_string(value))?;
        Ok(())
    }
}

impl Decode for AnyDataComponent {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(*DataComponent::from_id(r.read_varint()?).unwrap())
    }
}

impl Encode for AnyDataComponent {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.id())?;
        Ok(())
    }
}

impl Decode for Slot {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let item_count = r.read_varint()?;
        let item_id = if item_count > 0 {
            Some(r.read_varint()?)
        } else {
            None
        };

        let mut to_add: ComponentMap = ComponentMap::with_hasher(Default::default());
        let mut to_remove: Vec<i32> = vec![];
        if item_count > 0 {
            let n1 = r.read_varint()?;
            let n2 = r.read_varint()?;

            for _ in 0..n1 {
                let component = AnyDataComponent::decode(r)?;

                let id = component.id();
                let value = component.decode_value(r)?;
                to_add.insert(id, value);
            }

            for _ in 0..n2 {
                to_remove.push(r.read_varint()?);
            }
        }

        Ok(Self {
            item_count,
            item_id,
            to_add,
            to_remove,
        })
    }
}

impl Encode for Slot {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.item_count)?;
        if this.item_count > 0 {
            w.write_varint(this.item_id.unwrap())?;

            w.write_varint(this.to_add.len() as i32)?;
            w.write_varint(this.to_remove.len() as i32)?;

            for (id, data) in &this.to_add {
                let component = DataComponent::from_id(*id).unwrap();
                w.write_varint(*id)?;
                component.encode_value(w, data)?;
            }

            for id in &this.to_remove {
                w.write_varint(*id)?;
            }
        }
        Ok(())
    }
}
