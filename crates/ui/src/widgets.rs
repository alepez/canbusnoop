use dioxus::core::{Element, Scope};
use dioxus::core_macro::{render, Props};
use dioxus::events::MouseEvent;
use dioxus::prelude::EventHandler;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    render! {
        button {
            onclick: move |evt| cx.props.on_click.call(evt),
            class: "btn",
            &cx.props.children
        }
    }
}

#[derive(Props)]
pub struct TextInputProps<'a> {
    on_input: EventHandler<'a, Event<FormData>>,
    label: &'a str,
    placeholder: &'a str,
    value: &'a str,
}

pub fn TextInput<'a>(cx: Scope<'a, TextInputProps<'a>>) -> Element<'a> {
    render! {
        label {
            class: "form-control w-full max-w-xs",
            div {
                class: "label",
                span {
                    class: "label-text",
                    cx.props.label
                }
            }
            input {
                placeholder: cx.props.placeholder,
                class: "input input-bordered w-full max-w-xs",
                oninput: move |evt| cx.props.on_input.call(evt),
                value: cx.props.value,
            }
        }
    }
}
