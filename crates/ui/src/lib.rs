#![allow(non_snake_case)]

use canbusnoop_core::Frame;
use canbusnoop_db::MultiStats;
use dioxus::prelude::*;
use dioxus_desktop::Config;
use futures::StreamExt;
use log::info;
use std::cell::Cell;

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

    cx.render(rsx! {
        div {
            "Count: {count}"
        }
    })
}
