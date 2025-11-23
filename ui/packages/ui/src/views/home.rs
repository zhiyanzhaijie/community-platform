use crate::Route;
use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "flex flex-col items-center justify-center py-20 text-center",
            h1 {
                class: "text-5xl font-bold text-gray-900 mb-6",
                "Welcome to Community Platform"
            }
            p {
                class: "text-xl text-gray-600 mb-10 max-w-2xl",
                "Discover tools, connect with members, and build something amazing together."
            }
            div {
                class: "flex gap-4",
                Button {
                    size: ButtonSize::Large,
                    on_click: move |_| { nav.push(Route::ToolList {}); },
                    "Browse Tools"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Large,
                    on_click: move |_| { nav.push(Route::Login {}); },
                    "Get Started"
                }
            }
        }
    }
}
