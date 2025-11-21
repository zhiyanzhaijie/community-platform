use crate::{components::ButtonVariantsExample, Route};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { "HOme"}
        ButtonVariantsExample {  }
    }
}
