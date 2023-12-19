mod demo;

use anyhow::Result;
use canbusnoop_core::Frame;
use demo::DemoBusReader;
use tokio_socketcan::{CANFrame, CANSocket};
use tokio_stream::StreamExt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("socket can error")]
    SocketCanError(#[from] tokio_socketcan::Error),
    #[error("invalid interface: {0}")]
    InvalidInterface(String),
}

pub struct CanBusReader {
    inner: InnerCanBusReader,
}

impl CanBusReader {
    pub fn new(config: Config) -> Result<CanBusReader> {
        let inner = match config {
            Config::SocketCan(socket_can_config) => {
                InnerCanBusReader::SocketCan(SocketCanBusReader::new(socket_can_config)?)
            }
            Config::Demo => {
                InnerCanBusReader::Demo(DemoBusReader::new()?)
            }
        };
        Ok(CanBusReader { inner })
    }

    pub async fn read(&mut self) -> Option<Frame> {
        match &mut self.inner {
            InnerCanBusReader::SocketCan(socket_can) => socket_can.read().await,
            InnerCanBusReader::Demo(inner) => inner.read().await,
        }
    }
}

#[derive(Debug)]
pub enum Config {
    SocketCan(SocketCanConfig),
    Demo,
}

impl Config {
    pub fn new(interface: String) -> Result<Config, Error> {
        if interface.starts_with("can") {
            return Ok(Config::SocketCan(SocketCanConfig { interface }));
        }

        if interface.starts_with("demo") {
            return Ok(Config::Demo);
        }

        Err(Error::InvalidInterface(interface))
    }
}

#[derive(Debug)]
pub struct SocketCanConfig {
    interface: String,
}

enum InnerCanBusReader {
    SocketCan(SocketCanBusReader),
    Demo(DemoBusReader),
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
        let frame: Frame = socket_can_frame_to_frame(frame?.ok()?);
        Some(frame)
    }
}

fn socket_can_frame_to_frame(frame: CANFrame) -> Frame {
    Frame::new(frame.id(), frame.data().to_vec())
}
