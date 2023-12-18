use canbusnoop_core::Frame;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
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

impl Stats {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn last_period(&self) -> Option<Duration> {
        self.last_period
    }

    pub fn min_period(&self) -> Option<Duration> {
        self.min_period
    }

    pub fn max_period(&self) -> Option<Duration> {
        self.max_period
    }

    pub fn avg_period(&self) -> Option<Duration> {
        self.avg_period
    }

    pub fn throughput(&self) -> f64 {
        self.throughput
    }

    pub fn period_jitter(&self) -> f64 {
        self.period_jitter
    }
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

        let time_since_start = now - self.started_at;
        self.throughput = match self.count {
            0 => 0.,
            1 => 1.,
            _ => self.count as f64 / time_since_start.as_secs_f64(),
        };

        self.period_jitter = calculate_jitter(self.period_history.iter());
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MultiStats {
    stats: HashMap<u32, Stats>,
    total_count: usize,
}

impl MultiStats {
    pub fn push(&mut self, frame: Frame) {
        self.total_count += 1;

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

    pub fn count(&self) -> usize {
        self.total_count
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Stats)> {
        self.stats.iter()
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
