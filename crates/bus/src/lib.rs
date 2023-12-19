mod demo;
mod socket_can;

use anyhow::Result;
use canbusnoop_core::Frame;

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
            Config::SocketCan(cfg) => InnerCanBusReader::SocketCan(socket_can::Reader::new(cfg)?),
            Config::Demo => InnerCanBusReader::Demo(demo::Reader::new()),
        };
        Ok(CanBusReader { inner })
    }

    pub async fn read(&mut self) -> Option<Frame> {
        match &mut self.inner {
            InnerCanBusReader::SocketCan(inner) => inner.read().await,
            InnerCanBusReader::Demo(inner) => inner.read().await,
        }
    }
}

#[derive(Debug)]
pub enum Config {
    SocketCan(socket_can::Config),
    Demo,
}

impl Config {
    pub fn new(interface: String) -> Result<Config, Error> {
        if interface.starts_with("can") {
            return Ok(Config::SocketCan(socket_can::Config { interface }));
        }

        if interface.starts_with("demo") {
            return Ok(Config::Demo);
        }

        Err(Error::InvalidInterface(interface))
    }
}

enum InnerCanBusReader {
    SocketCan(socket_can::Reader),
    Demo(demo::Reader),
}
