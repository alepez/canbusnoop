#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    time::{Duration, Instant},
};

use canbusnoop_bus::{CanBusReader, Config, Frame};

#[derive(Debug)]
struct Stats {
    started_at: Instant,
    count: usize,
    last_time: Option<Instant>,
    last_period: Option<Duration>,
    min_period: Option<Duration>,
    max_period: Option<Duration>,
    avg_period: Option<Duration>,
    throughput: f64,
    period_history: VecDeque<Duration>,
    period_jitter: f64,
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
            avg_period: Default::default(),
            throughput: 0.,
            period_history: Default::default(),
            period_jitter: 0.,
        }
    }
}

fn fmt_period(x: Duration) -> String {
    let ms = x.as_millis();
    format!("{:6?}", ms)
}

struct Spinner(usize);

impl Display for Spinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 % 4 {
            0 => '|',
            1 => '/',
            2 => '-',
            3 => '\\',
            _ => unreachable!(),
        };
        write!(f, "{}", c)
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spinner = Spinner(self.count);
        write!(
            f,
            "({} {:6}, {:6}, {:6}, {:6}, {:6}, {:6.1}Hz, {:6.1}, {:6.1}%)",
            spinner,
            self.count,
            self.last_period.map(fmt_period).unwrap_or_default(),
            self.min_period.map(fmt_period).unwrap_or_default(),
            self.max_period.map(fmt_period).unwrap_or_default(),
            self.avg_period.map(fmt_period).unwrap_or_default(),
            self.avg_period
                .map(|x| x.as_secs_f64())
                .and_then(|s| if s != 0. { Some(1. / (s as f64)) } else { None })
                .unwrap_or_default(),
            self.throughput,
            self.period_jitter * 100.,
        )
    }
}

impl Stats {
    fn push(&mut self, frame: Frame) {
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

            if self.period_history.len() > 10 {
                self.period_history.pop_back();
            }

            self.period_history.push_back(last_period);

            if !self.period_history.is_empty() {
                let sum: u64 = self
                    .period_history
                    .iter()
                    .map(|x| x.as_millis() as u64)
                    .sum();
                let n: u64 = self.period_history.len() as _;
                self.avg_period = Some(Duration::from_millis(sum / n));
            }
        }

        self.throughput = (self.count as f64) / (now - self.started_at).as_secs_f64();
        self.period_jitter = calculate_jitter(self.period_history.iter());
    }
}

#[derive(Debug, Default)]
struct MultiStats {
    stats: HashMap<u32, Stats>,
}

impl MultiStats {
    fn push(&mut self, frame: Frame) {
        let id = frame.id();

        let s = self.stats.get_mut(&id);
        if let Some(s) = s {
            s.push(frame)
        } else {
            let mut s = Stats::default();
            s.push(frame);
            self.stats.insert(id, s);
        };
    }
}

impl Display for MultiStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = &self.stats;
        let mut stats: Vec<_> = stats.into_iter().collect();
        stats.sort_by_key(|(&k, _)| k);
        for (k, v) in stats {
            let data_page = (k >> 24) & 1;
            let pdu_format = (k >> 16) & 0xFF;
            let pdu_specific = (k >> 8) & 0xFF;
            let pgn = (data_page << 16) + (pdu_format << 8) + pdu_specific;
            let _ = writeln!(f, "0x{:08X} PGN={:8} {}", k, pgn, v);
        }
        Ok(())
    }
}

fn filter_id(id: u32, expected_id: u32) -> bool {
    id == expected_id
}

fn filter_src(id: u32, expected_src: u32) -> bool {
    let src = id & 0xFF;
    src == expected_src
}

fn calculate_jitter<'a>(mut periods: impl Iterator<Item = &'a Duration>) -> f64 {
    let mut previous_period = match periods.next() {
        Some(period) => period.as_secs_f64(),
        None => return 0.0, // Return 0 if there are no periods
    };
    let mut differences = Vec::new();
    // Calculate the differences between adjacent periods
    for period in periods {
        let period = period.as_secs_f64();
        let difference = (period - previous_period).abs();
        differences.push(difference);
        previous_period = period;
    }
    // Calculate the average absolute difference
    let sum: f64 = differences.iter().sum();
    if differences.is_empty() {
        return 0.0;
    }
    let average_difference = sum / (differences.len() as f64);
    // Calculate and return the jitter value
    average_difference
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
