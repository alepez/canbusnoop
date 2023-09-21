use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut socket_rx = CANSocket::open("can0")?;

    while let Some(Ok(frame)) = socket_rx.next().await {
        let data = frame.data();
        let id = frame.id();

    }
    Ok(())
}
