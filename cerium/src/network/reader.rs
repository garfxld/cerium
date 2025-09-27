use std::{
    io::{Error, ErrorKind},
    pin::{Pin, pin},
    task::{Context, Poll},
};

use aes::cipher::{BlockDecryptMut as _, BlockSizeUser as _, KeyIvInit as _};
use async_compression::tokio::bufread::ZlibDecoder;

use tokio::io::{AsyncRead, AsyncReadExt, BufReader, ReadBuf};

use crate::{auth::Decryptor, protocol::packet::RawPacket};

pub struct StreamReader<R>
where
    R: AsyncRead + Unpin,
{
    inner: BufReader<R>,

    cipher: Option<Decryptor>,
    threshold: i32,
}

impl<R> StreamReader<R>
where
    R: AsyncRead + Unpin,
{
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),

            cipher: None,
            threshold: -1,
        }
    }

    pub fn set_encryption(&mut self, shared_secret: &[u8]) {
        let decryptor = Decryptor::new_from_slices(shared_secret, shared_secret).unwrap();
        self.cipher = Some(decryptor);
    }

    pub fn set_compression(&mut self, threshold: i32) {
        self.threshold = threshold;
    }

    #[inline]
    fn varint_size(value: i32) -> i32 {
        match value {
            0 => 1,
            n => ((31 - n.leading_zeros()) / 7 + 1) as i32,
        }
    }

    pub async fn read_packet(&mut self) -> Result<RawPacket, ()> {
        let packet_len = match self.read_varint().await {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        let compressed = self.threshold != -1;
        let mut take = (self).take(packet_len as u64);
        if !compressed {
            // WITHOUT compression

            let id = take.read_varint().await.unwrap();

            let mut data = Vec::with_capacity(take.limit() as usize);
            take.read_to_end(&mut data).await.unwrap();

            return Ok(RawPacket::new(id, data));
        } else {
            // WITH compression

            let data_len = take.read_varint().await.unwrap();

            let mut payload = vec![0; (packet_len - Self::varint_size(data_len)) as usize];
            take.read_exact(&mut payload).await.unwrap();

            let mut data = Vec::new();
            if data_len > 0 {
                // size >= threshold

                let mut inflator = ZlibDecoder::new(BufReader::new(&payload[..]));
                inflator.read_to_end(&mut data).await.unwrap();
            } else {
                // size < threshold

                data = payload;
            }

            let mut data = &data[..];

            let id = data.read_varint().await.unwrap();

            return Ok(RawPacket::new(id, data.to_vec()));
        }
    }
}

impl<W> AsyncRead for StreamReader<W>
where
    W: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let reader = pin!(&mut this.inner);

        match &mut this.cipher {
            // Unencrypted
            None => reader.poll_read(cx, buf),

            // Encrypted
            Some(cipher) => {
                let len = buf.filled().len();
                let poll = reader.poll_read(cx, buf);

                if poll.is_ready() {
                    for block in buf.filled_mut()[len..].chunks_mut(Decryptor::block_size()) {
                        cipher.decrypt_block_mut(block.into());
                    }
                }
                poll
            }
        }
    }
}

trait AsyncReadExtExt
where
    Self: AsyncReadExt + Unpin,
{
    async fn read_varint(&mut self) -> Result<i32, Error>;
}

impl<T> AsyncReadExtExt for T
where
    T: AsyncReadExt + Unpin,
{
    async fn read_varint(&mut self) -> Result<i32, Error> {
        let mut value = 0;
        for i in 0..5 {
            let byte = self.read_u8().await?;
            value |= (i32::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(value);
            }
        }
        return Err(ErrorKind::Other.into());
    }
}
