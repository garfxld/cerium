use bytes::BytesMut;

pub trait Encode<T> {
    fn encode(write: &mut BytesMut, value: Self) -> anyhow::Result<()>;
}