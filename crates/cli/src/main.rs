#![allow(dead_code)]

use canbusnoop_bus::{CanBusReader, Config};
use canbusnoop_db::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
