#![allow(non_snake_case)]

mod stats;
mod stats_item;

use canbusnoop_core::Frame;
use canbusnoop_db::MultiStats;
use dioxus::prelude::*;
use dioxus_desktop::Config;
use futures::StreamExt;
use stats::Stats;
use std::cell::Cell;

struct AppProps {
    rx_receiver: Cell<Option<UnboundedReceiver<Frame>>>,
}

pub fn launch(rx_receiver: UnboundedReceiver<Frame>) {
    let rx_receiver = Cell::new(Some(rx_receiver));
    let props = AppProps { rx_receiver };
    let config = Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string());

    dioxus_desktop::launch_with_props(App, props, config);
}

fn App(cx: Scope<AppProps>) -> Element {
    let stats = use_ref(cx, || MultiStats::default());
    let can_id_filter = use_state(cx, || "00000000".to_string());
    let can_id_mask = use_state(cx, || "00000000".to_string());

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

    let clear = || {
        stats.write().clear();
    };

    let count = stats.read().count();
    let stats: MultiStats = stats.read().clone();

    let stats = {
        let can_id_filter = u32::from_str_radix(can_id_filter.as_str(), 16).unwrap_or(0x00000000);
        let can_id_mask = u32::from_str_radix(can_id_mask.as_str(), 16).unwrap_or(0x00000000);
        stats.filter_by_can_id(can_id_filter, can_id_mask)
    };

    render! {
        button { onclick: move |_event| { clear() }, "Clear" }
        div {
            "Total: {count}"
        }
        div {
          div { "filter" }
          input {
            value: "{can_id_filter}",
            oninput: move |evt| can_id_filter.set(evt.value.clone()),
          }
          div { "mask" }
          input {
            value: "{can_id_mask}",
            oninput: move |evt| can_id_mask.set(evt.value.clone()),
          }
        }
        Stats {
            stats: stats
        }
    }
}
