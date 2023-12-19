#![allow(non_snake_case)]

use canbusnoop_core::Frame;
use canbusnoop_db::{MultiStats, Stats};
use colorsys::{Hsl, Rgb};
use dioxus::prelude::*;
use dioxus_desktop::Config;
use futures::StreamExt;
use std::cell::Cell;
use std::time::Duration;

struct AppProps {
    rx_receiver: Cell<Option<UnboundedReceiver<Frame>>>,
}

pub fn launch(rx_receiver: UnboundedReceiver<Frame>) {
    let rx_receiver = Cell::new(Some(rx_receiver));
    let props = AppProps { rx_receiver };
    let config = Config::default();
    dioxus_desktop::launch_with_props(App, props, config);
}

fn App(cx: Scope<AppProps>) -> Element {
    let stats = use_ref(cx, || MultiStats::default());

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let receiver = cx.props.rx_receiver.take();
        to_owned![stats];
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.next().await {
                    let msg: Frame = msg;
                    stats.write().push(msg);
                }
            }
        }
    });

    let count = stats.read().count();
    let stats: MultiStats = stats.read().clone();

    cx.render(rsx! {
        div {
            "Total: {count}"
        }
        Stats {
            stats: stats
        }
    })
}

#[derive(Props, PartialEq)]
struct StatsProps {
    stats: MultiStats,
}

fn Stats(cx: Scope<StatsProps>) -> Element {
    let stats = &cx.props.stats;

    cx.render(rsx! {
        table {
            width: "100%",
            thead {
                tr {
                    th { width: "20%", text_align: "right", "ID" }
                    th { width: "10%", text_align: "right", "Count" }
                    th { width: "10%", text_align: "right", "Last" }
                    th { width: "10%", text_align: "right", "Min" }
                    th { width: "10%", text_align: "right", "Max" }
                    th { width: "10%", text_align: "right", "Avg" }
                    th { width: "10%", text_align: "right", "Freq" }
                    th { width: "10%", text_align: "right", "Throughput" }
                    th { width: "10%", text_align: "right", "Jitter" }
                }
                tr {
                    th { text_align: "right", "" }
                    th { text_align: "right", "" }
                    th { text_align: "right", "(ms)" }
                    th { text_align: "right", "(ms)" }
                    th { text_align: "right", "(ms)" }
                    th { text_align: "right", "(ms)" }
                    th { text_align: "right", "(Hz)" }
                    th { text_align: "right", "(Hz)" }
                    th { text_align: "right", "%" }
                }
            }
            tbody {
                for (&id, stats) in stats.iter() {
                    StatsItem {
                        id: id,
                        stats: stats.clone()
                    }
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct StatsItemProps {
    id: u32,
    stats: Stats,
}

fn StatsItem(cx: Scope<StatsItemProps>) -> Element {
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
                font_family: "monospace",
                font_size: "1.2em",
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
            background_color: "{tr_bg_color}",
            onmouseover: move |_| over.set(true),
            onmouseleave: move |_| over.set(false),
            td { text_align: "right", id }
            td { text_align: "right", count }
            td { text_align: "right", last_period }
            td { text_align: "right", min_period }
            td { text_align: "right", max_period }
            td { text_align: "right", avg_period }
            td { text_align: "right", avg_freq }
            td { text_align: "right", throughput }
            td { text_align: "right", period_jitter }
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
