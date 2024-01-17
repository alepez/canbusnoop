use canbusnoop_db::Stats;
use colorsys::{Hsl, Rgb};
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, PartialEq)]
pub(crate) struct StatsItemProps {
    id: u32,
    stats: Stats,
}

pub(crate) fn StatsItem(cx: Scope<StatsItemProps>) -> Element {
    let stats = &cx.props.stats;
    let id = cx.props.id;

    let stats_str = StatsStrings::from(stats);

    render! {
        Row {
            Cell { ColoredId { id: id } }
            Cell { stats_str.count }
            Cell { stats_str.last_period }
            Cell { stats_str.min_period }
            Cell { stats_str.max_period }
            Cell { stats_str.avg_period }
            Cell { stats_str.avg_freq }
            Cell { stats_str.throughput }
            Cell { stats_str.period_jitter }
        }
    }
}

fn fmt_period(x: Duration) -> String {
    let ms = x.as_millis();
    format!("{:6?}", ms)
}

/// Translate a nibble (0-16) to a color hex string
fn nibble_to_color(byte: u8) -> Rgb {
    let h = byte as f64 / 16. * (360. / 16.0 * 15.0);
    let s = 100.;
    let l = 50.;
    let color = Hsl::new(h, s, l, None);
    Rgb::from(color)
}

fn luminance(c: &Rgb) -> f64 {
    const R_YUV_FACTOR: f64 = 0.299;
    const G_YUV_FACTOR: f64 = 0.587;
    const B_YUV_FACTOR: f64 = 0.114;
    let r = c.red() * R_YUV_FACTOR;
    let g = c.green() * G_YUV_FACTOR;
    let b = c.blue() * B_YUV_FACTOR;
    r + g + b
}

fn text_color_from_bg(bg: &Rgb) -> Rgb {
    if luminance(bg) > 50. {
        Rgb::new(0., 0., 0., None)
    } else {
        Rgb::new(255., 255., 255., None)
    }
}

struct StatsStrings {
    count: String,
    last_period: String,
    min_period: String,
    max_period: String,
    avg_period: String,
    throughput: String,
    period_jitter: String,
    avg_freq: String,
}

impl From<&Stats> for StatsStrings {
    fn from(stats: &Stats) -> Self {
        let count = stats.count().to_string();
        let last_period = stats.last_period().map(fmt_period).unwrap_or_default();
        let min_period = stats.min_period().map(fmt_period).unwrap_or_default();
        let max_period = stats.max_period().map(fmt_period).unwrap_or_default();
        let avg_period = stats.avg_period().map(fmt_period).unwrap_or_default();

        let avg_freq = stats.avg_period().map(|x| x.as_secs_f64()).and_then(|s| {
            if s != 0. {
                Some(1. / (s as f64))
            } else {
                None
            }
        });

        let avg_freq = match avg_freq {
            Some(avg_freq) => format!("{:.2}", avg_freq),
            None => "".to_string(),
        };

        let throughput = stats.throughput();
        let throughput = format!("{:.2}", throughput);

        let period_jitter = stats.period_jitter() * 100.;
        let period_jitter = format!("{:.2}", period_jitter);

        Self {
            count,
            last_period,
            min_period,
            max_period,
            avg_period,
            throughput,
            period_jitter,
            avg_freq,
        }
    }
}

#[component]
fn ColoredId(cx: Scope, id: u32) -> Element {
    let id_arr = id.to_be_bytes();

    render! {
        div {
            for &c in id_arr.iter() {
                ColoredNibble { nibble: c >> 4 }
                ColoredNibble { nibble: c & 0x0F }
            }
        }
    }
}

#[component]
fn ColoredNibble(cx: Scope, nibble: u8) -> Element {
    let bg_color = nibble_to_color(*nibble);
    let fg_color = text_color_from_bg(&bg_color);
    let nibble = format!("{:01X}", nibble);

    let bg_color = bg_color.to_hex_string();
    let fg_color = fg_color.to_hex_string();

    render! {
        span {
            background_color: "{bg_color}",
            color: "{fg_color}",
            padding: "0.2em",
            nibble
        }
    }
}

#[derive(Props)]
struct CellProps<'a> {
    children: Element<'a>,
}

fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element {
    render!(
        td {
            class: "p-2",
            &cx.props.children
        }
    )
}

#[derive(Props)]
struct RowProps<'a> {
    children: Element<'a>,
}

fn Row<'a>(cx: Scope<'a, RowProps<'a>>) -> Element {
    render! {
        tr {
            class: "bg-white hover:bg-gray-200 border-b dark:bg-gray-800 dark:border-gray-700",
            &cx.props.children
        }
    }
}
