use anyhow::Result;
use canbusnoop_core::Frame;
use tokio_socketcan::{CANFrame, CANSocket};
use tokio_stream::StreamExt;

pub(super) struct Reader {
    socket: CANSocket,
}

impl Reader {
    pub(super) fn new(config: Config) -> Result<Reader> {
        let socket = CANSocket::open(config.interface.as_str())?;
        Ok(Reader { socket })
    }

    pub(super) async fn read(&mut self) -> Option<Frame> {
        let frame = self.socket.next().await;
        let frame: Frame = socket_can_frame_to_frame(frame?.ok()?);
        Some(frame)
    }
}

fn socket_can_frame_to_frame(frame: CANFrame) -> Frame {
    Frame::new(frame.id(), frame.data().to_vec())
}

#[derive(Debug)]
pub struct Config {
    pub(super) interface: String,
}
