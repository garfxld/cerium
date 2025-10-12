use std::{
    io::Error,
    pin::{Pin, pin},
    task::{Context, Poll},
};

use aes::cipher::{BlockEncryptMut as _, BlockSizeUser as _, KeyIvInit as _};
use async_compression::{Level, tokio::write::ZlibEncoder};

use crate::protocol::encode::EncodeError;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::auth::Encryptor;

pub struct StreamWriter<W>
where
    W: AsyncWrite + Unpin,
{
    inner: BufWriter<W>,

    cipher: Option<Encryptor>,
    threshold: i32,
}

impl<W> StreamWriter<W>
where
    W: AsyncWrite + Unpin,
{
    pub fn new(inner: W) -> Self {
        Self {
            inner: BufWriter::new(inner),

            cipher: None,
            threshold: -1,
        }
    }

    pub fn set_encryption(&mut self, shared_secret: &[u8]) {
        let encryptor = Encryptor::new_from_slices(shared_secret, shared_secret).unwrap();
        self.cipher = Some(encryptor);
    }

    pub fn set_compression(&mut self, threshold: i32) {
        self.threshold = threshold;
    }

    async fn write_varint(&mut self, value: i32) -> Result<(), EncodeError> {
        let mut val = value;
        for _ in 0..5 {
            let b: u8 = val as u8 & 0b01111111;
            val >>= 7;
            self.write_u8(if val == 0 { b } else { b | 0b10000000 })
                .await
                .map_err(|_| EncodeError::Encode("Failed to encode VarInt".to_string()))?;
            if val == 0 {
                break;
            }
        }
        Ok(())
    }

    #[inline]
    fn varint_size(value: i32) -> i32 {
        match value {
            0 => 1,
            n => ((31 - n.leading_zeros()) / 7 + 1) as i32,
        }
    }

    pub async fn write_packet(&mut self, packet: &[u8]) -> Result<(), EncodeError> {
        let compressed = self.threshold != -1;

        let mut data = packet;
        let data_len = data.len() as i32;

        if !compressed {
            // WITHOUT compression

            self.write_varint(data_len).await?;
            self.write_all(&mut data)
                .await
                .map_err(|e| EncodeError::IoError(e))?;
        } else {
            // WITH compression

            if data_len >= self.threshold {
                // size >= threshold

                let mut compressed = Vec::new();

                let mut deflator = ZlibEncoder::with_quality(&mut compressed, Level::Default);
                deflator
                    .write_all(&data)
                    .await
                    .map_err(|e| EncodeError::IoError(e))?;
                deflator
                    .flush()
                    .await
                    .map_err(|e| EncodeError::IoError(e))?;

                // len of data_len + compressed_len
                self.write_varint(Self::varint_size(data_len) + compressed.len() as i32)
                    .await?;

                // data_len
                self.write_varint(data_len).await?;
                self.write_all(&compressed)
                    .await
                    .map_err(|e| EncodeError::IoError(e))?;
            } else {
                // size < threshold

                // len of data_len + data_len (because uncompressed)
                self.write_varint(1 + data_len).await?;

                // data_len (0 to indicate uncompressed)
                self.write_varint(0).await?;

                self.write_all(&data)
                    .await
                    .map_err(|e| EncodeError::IoError(e))?;
            }
        }

        self.flush().await.map_err(|e| EncodeError::IoError(e))
    }
}

impl<W> AsyncWrite for StreamWriter<W>
where
    W: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let this = self.get_mut();

        match &mut this.cipher {
            // Unencrypted
            None => pin!(&mut this.inner).poll_write(cx, buf),

            // Encrypted
            Some(cipher) => {
                let mut total_written = 0;

                for block in buf.chunks(Encryptor::block_size()) {
                    let mut buf = [0u8];
                    cipher.encrypt_block_b2b_mut(block.into(), (&mut buf).into());

                    match pin!(&mut this.inner).poll_write(cx, &buf) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(result) => match result {
                            Ok(written) => total_written += written,
                            Err(e) => return Poll::Ready(Err(e)),
                        },
                    }
                }
                Poll::Ready(Ok(total_written))
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let writer = pin!(&mut self.get_mut().inner);
        writer.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let writer = pin!(&mut self.get_mut().inner);
        writer.poll_shutdown(cx)
    }
}
