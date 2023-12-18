#![allow(dead_code)]

use canbusnoop_bus::{CanBusReader, Config};
use canbusnoop_core::Frame;
use canbusnoop_db::*;
use canbusnoop_ui::launch;
use futures_channel::mpsc::{unbounded, UnboundedSender};

#[tokio::main]
async fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new_socket_can("can0".to_string());
    let mut reader = CanBusReader::new(config)?;

    let mut stats = MultiStats::default();

    while let Some(frame) = reader.read().await {
        stats.push(frame);

        clear_screen();
        println!("{}", stats);
    }

    Ok(())
}

fn sniff_background_task(
    rx_sender: UnboundedSender<Frame>,
) -> Result<(), Box<dyn std::error::Error>> {
    // let config = Config::new_socket_can("can0".to_string());
    // let mut reader = CanBusReader::new(config)?;
    //
    // while let Some(frame) = reader.read().await {
    //     rx_sender.unbounded_send(frame)?;
    // }

    loop {
        let frame = Frame::new(0x123, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        rx_sender.unbounded_send(frame)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_env_logger();

    let (rx_sender, rx_receiver) = unbounded::<Frame>();

    std::thread::spawn(move || {
        let _ = sniff_background_task(rx_sender);
    });

    launch(rx_receiver);

    Ok(())
}

fn setup_env_logger() {
    use env_logger::{Builder, Target};

    let mut builder = Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.target(Target::Stdout);
    builder.init();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
