use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

#[derive(Debug)]
struct Stats {
    started_at: Instant,
    count: usize,
    last_time: Option<Instant>,
    last_period: Option<Duration>,
    min_period: Option<Duration>,
    max_period: Option<Duration>,
    throughput: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
            count: Default::default(),
            last_time: Default::default(),
            last_period: Default::default(),
            min_period: Default::default(),
            max_period: Default::default(),
            throughput: 0.,
        }
    }
}

fn fmt_period(x: Duration) -> String {
    format!("{:?}", x)
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:10}, {:10}, {:10}, {:10}, {:10.1})",
            self.count,
            self.last_period.map(fmt_period).unwrap_or_default(),
            self.min_period.map(fmt_period).unwrap_or_default(),
            self.max_period.map(fmt_period).unwrap_or_default(),
            self.throughput,
        )
    }
}

impl Stats {
    fn push(&mut self, frame: tokio_socketcan::CANFrame) {
        let id = frame.id();

        log::debug!("{:?}", &frame);

        let now = Instant::now();

        self.count += 1;
        self.last_period = self.last_time.map(|last_time| now - last_time);
        self.last_time = Some(now);

        if let Some(last_period) = self.last_period {
            self.min_period = Some(
                self.min_period
                    .map(|x| x.min(last_period))
                    .unwrap_or(last_period),
            );
            self.max_period = Some(
                self.max_period
                    .map(|x| x.max(last_period))
                    .unwrap_or(last_period),
            );
        }

        self.throughput = (self.count as f64) / (now - self.started_at).as_secs_f64();

        log::info!("{:08X} {}", id, self);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    {
        use env_logger::{Builder, Target};

        let mut builder = Builder::from_default_env();
        builder.format_timestamp_millis();
        builder.target(Target::Stdout);
        builder.init();
    }

    let mut socket_rx = CANSocket::open("can0")?;
    let mut stats = Stats::default();

    while let Some(Ok(frame)) = socket_rx.next().await {
        stats.push(frame);
    }
    Ok(())
}
