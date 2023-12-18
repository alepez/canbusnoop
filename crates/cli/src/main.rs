#![allow(dead_code)]

use canbusnoop_bus::{CanBusReader, Config};
use canbusnoop_core::Frame;
use canbusnoop_ui::launch;
use clap::Parser;
use futures_channel::mpsc::{unbounded, UnboundedSender};

async fn can_read_task(
    can_interface: String,
    rx_sender: UnboundedSender<Frame>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new_socket_can(can_interface);
    let mut reader = CanBusReader::new(config)?;

    while let Some(frame) = reader.read().await {
        rx_sender.unbounded_send(frame).unwrap();
    }

    Ok(())
}

fn can_read_thread_fun(can_interface: String, rx_sender: UnboundedSender<Frame>) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            can_read_task(can_interface, rx_sender).await.unwrap();
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let can_interface = cli.can_interface;

    setup_env_logger();

    let (rx_sender, rx_receiver) = unbounded::<Frame>();

    std::thread::spawn(move || {
        let _ = can_read_thread_fun(can_interface, rx_sender);
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// CAN bus interface
    #[arg(short = 'i', long, default_value = "can0")]
    can_interface: String,
}
