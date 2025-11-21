use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant};

#[component]
pub fn ButtonVariantsExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2.5 items-center",
            Button {
                variant: ButtonVariant::Primary,
                "Primary"
            }

            Button {
                variant: ButtonVariant::Secondary,
                "Secondary"
            }

            Button {
                variant: ButtonVariant::Outline,
                "Outline"
            }

            Button {
                variant: ButtonVariant::Ghost,
                "Ghost"
            }

            Button {
                variant: ButtonVariant::Link,
                "Link"
            }

            Button {
                variant: ButtonVariant::Destructive,
                "Destructive"
            }
        }
    }
}
