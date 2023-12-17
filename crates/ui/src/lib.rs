#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn launch() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut started = use_state(cx, || false);

    if !started {
        started.set(true);
        let _ = tokio::spawn(async {
            let _ = tokio::task::spawn_local(async {
                // some !Send work
            })
            .await;
        });
    }

    cx.render(rsx! {
        div {
            "Ciao!"
        }
    })
}
