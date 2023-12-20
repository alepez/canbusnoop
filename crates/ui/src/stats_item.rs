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
    let over = use_state(cx, || false);
    let stats = &cx.props.stats;
    let id = cx.props.id;

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

    let id = {
        let id_arr = id.to_be_bytes();

        cx.render(rsx! {
            div {
                for &c in id_arr.iter() {
                    span {
                        background_color: "{nibble_to_color(c >> 4)}",
                        padding: "0.2em",
                        format!("{:01X}", c >> 4)
                    }
                    span {
                        background_color: "{nibble_to_color(c & 0x0F)}",
                        padding: "0.2em",
                        format!("{:01X}", c & 0x0F)
                    }
                }
            }
        })
    };

    let tr_bg_color = if *over.get() {
        "#c0c0c0"
    } else {
        "transparent"
    };

    cx.render(rsx! {
        tr {
            class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700",
            background_color: "{tr_bg_color}",
            onmouseover: move |_| over.set(true),
            onmouseleave: move |_| over.set(false),
            th {
                class: "p-2 font-mono font-medium text-gray-900 dark:text-white",
                id
            }
            td {
                class: "p-2",
                count
            }
            td {
                class: "p-2",
                last_period
            }
            td {
                class: "p-2",
                min_period
            }
            td {
                class: "p-2",
                max_period
            }
            td {
                class: "p-2",
                avg_period
            }
            td {
                class: "p-2",
                avg_freq
            }
            td {
                class: "p-2",
                throughput
            }
            td {
                class: "p-2",
                period_jitter
            }
        }
    })
}

fn fmt_period(x: Duration) -> String {
    let ms = x.as_millis();
    format!("{:6?}", ms)
}

/// Translate a nibble (0-16) to a color hex string
fn nibble_to_color(byte: u8) -> String {
    let h = byte as f64 / 16. * (360. / 16.0 * 15.0);
    let s = 100.;
    let l = 50.;
    let color = Hsl::new(h, s, l, None);
    Rgb::from(color).to_hex_string()
}
