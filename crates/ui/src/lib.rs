#![allow(non_snake_case)]

use canbusnoop_core::Frame;
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
    let count = use_state(cx, || 0);

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        let receiver = cx.props.rx_receiver.take();
        to_owned![count];
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.next().await {
                    info!("Received: {:?}", msg);
                    count += 1;
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            "Count: {count}"
        }
    })
}
