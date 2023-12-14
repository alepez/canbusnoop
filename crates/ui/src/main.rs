#![allow(dead_code)]

use canbusnoop_bus::{CanBusReader, Config};
use canbusnoop_db::*;

fn filter_id(id: u32, expected_id: u32) -> bool {
    id == expected_id
}

fn filter_src(id: u32, expected_src: u32) -> bool {
    let src = id & 0xFF;
    src == expected_src
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        use env_logger::{Builder, Target};

        let mut builder = Builder::from_default_env();
        builder.format_timestamp_millis();
        builder.target(Target::Stdout);
        builder.init();
    }

    let config = Config::new_socket_can("can0".to_string());
    let mut reader = CanBusReader::new(config)?;

    // let mut stats = Stats::default();
    let mut stats = MultiStats::default();

    while let Some(frame) = reader.read().await {
        let id = frame.id();
        if filter_src(id, 23) {
            // if filter_id(id, 0x09F11917) {
            stats.push(frame);

            print!("\x1B[2J\x1B[1;1H");
            println!("{}", stats);
        }
    }

    Ok(())
}
