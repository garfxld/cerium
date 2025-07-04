use bytes::BytesMut;


pub trait Decode<T> {
    fn decode(read: &mut BytesMut) -> anyhow::Result<T>;
}