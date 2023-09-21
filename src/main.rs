use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

#[derive(Default)]
struct Stats;

impl Stats {
    fn push(&mut self, frame: tokio_socketcan::CANFrame) {
        log::info!("{:?}", &frame);
        let _data = frame.data();
        let _id = frame.id();
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut socket_rx = CANSocket::open("can0")?;
    let mut stats = Stats::default();

    while let Some(Ok(frame)) = socket_rx.next().await {
        stats.push(frame);
    }
    Ok(())
}
