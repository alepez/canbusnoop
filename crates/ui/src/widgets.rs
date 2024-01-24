use dioxus::prelude::*;
use dioxus::core::{Element, Scope};
use dioxus::core_macro::{render, Props};
use dioxus::events::MouseEvent;
use dioxus::prelude::EventHandler;

#[derive(Props)]
pub struct ButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    render! {
        button {
            onclick: move |evt| cx.props.on_click.call(evt),
            class: "bg-teal-400 rounded-lg px-4 py-2 text-white",
            &cx.props.children
        }
    }
}
