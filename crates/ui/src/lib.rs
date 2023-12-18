#![allow(non_snake_case)]

use canbusnoop_core::Frame;
use canbusnoop_db::{MultiStats, Stats};
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
            thead {
                tr {
                    th { "ID" }
                    th { "Count" }
                    th { "Last" }
                    th { "Min" }
                    th { "Max" }
                    th { "Avg" }
                    th { "Throughput" }
                    th { "Jitter" }
                }
                tr {
                    th { "" }
                    th { "" }
                    th { "(ms)" }
                    th { "(ms)" }
                    th { "(ms)" }
                    th { "(ms)" }
                    th { "Hz" }
                    th { "%" }
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
    let stats = &cx.props.stats;
    let id = cx.props.id;

    let id = format!("{:08X}", id);
    let count = stats.count().to_string();
    let last_period = stats.last_period().map(fmt_period).unwrap_or_default();
    let min_period = stats.min_period().map(fmt_period).unwrap_or_default();
    let max_period = stats.max_period().map(fmt_period).unwrap_or_default();

    let avg_period = stats
        .avg_period()
        .map(|x| x.as_secs_f64())
        .and_then(|s| if s != 0. { Some(1. / (s as f64)) } else { None })
        .unwrap_or_default();
    let avg_period = format!("{:.2}", avg_period);

    let throughput = stats.throughput();
    let throughput = format!("{:.2}", throughput);

    let period_jitter = (stats.period_jitter() * 100.).to_string();
    let period_jitter = format!("{:.2}", period_jitter);

    cx.render(rsx! {
        tr {
            td { id }
            td { count }
            td { last_period }
            td { min_period }
            td { max_period }
            td { avg_period }
            td { throughput }
            td { period_jitter }
        }
    })
}

fn fmt_period(x: Duration) -> String {
    let ms = x.as_millis();
    format!("{:6?}", ms)
}
