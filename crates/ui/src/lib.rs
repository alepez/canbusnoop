#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn launch() {
    dioxus_tui::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
