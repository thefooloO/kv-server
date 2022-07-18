use crate::pb::abi::{CommandRequest, CommandResponse};
use crate::KvError;
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};

pub const LEN: usize = 4;
pub const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;

pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let size = self.encoded_len();
        if size > MAX_FRAME {
            return Err(KvError::FrameError);
        }
        buf.put_u32(size as u32);

        self.encode(buf)?;
        Ok(())
    }

    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError> {
        let len = buf.get_u32() as usize;
        let msg = Self::decode(&buf[..len])?;
        buf.advance(len);
        Ok(msg)
    }
}

impl FrameCoder for CommandRequest {}
impl FrameCoder for CommandResponse {}

pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), KvError>
where
    S: AsyncRead + Unpin + Send,
{
    let len = stream.read_u32().await? as usize;
    buf.reserve(LEN + len);
    buf.put_u32(len as u32);
    unsafe { buf.advance_mut(len) };
    stream.read_exact(&mut buf[LEN..]).await;
    Ok(())
}
