use anyhow::Result;
use tokio_socketcan::{CANFrame, CANSocket};
use tokio_stream::StreamExt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("socket can error")]
    SocketCanError(#[from] tokio_socketcan::Error),
}

pub struct CanBusReader {
    inner: InnerCanBusReader,
}

impl CanBusReader {
    pub fn new(config: Config) -> Result<CanBusReader> {
        let inner = match config {
            Config::SocketCan(socket_can_config) => InnerCanBusReader::SocketCan(
                SocketCanBusReader::new(socket_can_config)?,
            ),
        };
        Ok(CanBusReader { inner })
    }

    pub async fn read(&mut self) -> Option<Frame> {
        match &mut self.inner {
            InnerCanBusReader::SocketCan(socket_can) => socket_can.read().await,
        }
    }
}

pub enum Config {
    SocketCan(SocketCanConfig),
}

impl Config {
    pub fn new_socket_can(interface: String) -> Config {
        Config::SocketCan(SocketCanConfig { interface })
    }
}

pub struct SocketCanConfig {
    interface: String,
}

enum InnerCanBusReader {
    SocketCan(SocketCanBusReader),
}

struct SocketCanBusReader {
    socket: CANSocket,
}

impl SocketCanBusReader {
    fn new(config: SocketCanConfig) -> Result<SocketCanBusReader> {
        let socket = CANSocket::open(config.interface.as_str())?;
        Ok(SocketCanBusReader { socket })
    }

    async fn read(&mut self) -> Option<Frame> {
        let frame = self.socket.next().await;
        let frame: Frame = frame?.ok()?.into();
        Some(frame)
    }
}

/// CAN Frame
#[derive(Debug)]
pub struct Frame {
    /// 32 bit CAN_ID + EFF/RTR/ERR flags
    id: u32,

    /// buffer for data
    data: Vec<u8>,
}

impl Frame {
    /// Returns the 32 bit CAN_ID + EFF/RTR/ERR flags
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the data
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl From<CANFrame> for Frame {
    fn from(frame: CANFrame) -> Self {
        Frame {
            id: frame.id(),
            data: frame.data().to_vec(),
        }
    }
}
