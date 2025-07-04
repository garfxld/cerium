use bytes::{Buf as _, BufMut as _, BytesMut};

use crate::protocol::{decoder::Decode, encoder::Encode, packet::{HandshakePacket, PingRequestPacket, PongResponsePacket, StatusRequestPacket, StatusResponsePacket}};

pub struct VarInt(pub i32);

impl Decode<Self> for VarInt {
    fn decode(read: &mut BytesMut) -> anyhow::Result<Self> {
        let mut val = 0;
        for i in 0..5 {
            let byte = read.get_u8();
            val |= (i32::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarInt(val));
            }
        }
        anyhow::bail!("VarInt is too large")
    }
}

impl Encode<Self> for VarInt {
    fn encode(write: &mut BytesMut, value: Self) -> anyhow::Result<()> {
        let x = value.0 as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        write.put(unsafe { bytes.get_unchecked(..bytes_needed as usize) });

        Ok(())
    }
}

impl Decode<Self> for HandshakePacket {
    fn decode(read: &mut BytesMut) -> anyhow::Result<Self> {
        let protocol_version = VarInt::decode(read)?.0;
        let server_address = String::decode(read)?;
        let server_port = read.get_u16();
        let intent = VarInt::decode(read)?.0;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            intent,
        })
    }
}

impl Decode<Self> for StatusRequestPacket {
    fn decode(_: &mut BytesMut) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl Decode<Self> for String {
    fn decode(read: &mut BytesMut) -> anyhow::Result<Self> {
        let length = VarInt::decode(read)?.0 as usize;
        let bytes = read.split_to(length);

        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

impl Encode<Self> for String {
    fn encode(write: &mut BytesMut, value: Self) -> anyhow::Result<()> {
        let length = value.len() as i32;
        VarInt::encode(write, VarInt(length))?;

        write.put(value.as_bytes());
        Ok(())
    }
}

impl Encode<Self> for StatusResponsePacket {
    fn encode(write: &mut BytesMut, value: Self) -> anyhow::Result<()> {
        String::encode(write, value.json_response)?;

        Ok(())
    }
}

impl Decode<Self> for PingRequestPacket {
    fn decode(read: &mut BytesMut) -> anyhow::Result<Self> {
        let timestamp = read.get_i64();
        Ok(Self { timestamp })
    }
}

impl Encode<Self> for PongResponsePacket {
    fn encode(write: &mut BytesMut, value: Self) -> anyhow::Result<()> {
        write.put_i64(value.timestamp);
        Ok(())
    }
}

