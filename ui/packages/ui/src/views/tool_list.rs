use crate::io::list_tools;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ToolList() -> Element {
    let tools_data = use_server_future(move || list_tools(1, 20))?;

    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-3xl font-bold text-gray-900",
                "Tools"
            }
            p {
                class: "text-gray-600 mb-6",
                "Browse our collection of tools using the accordion below:"
            }

            match tools_data() {
                Some(data) => rsx! {
                    div {

                    }
                },
                Some(Err(e)) => rsx! {
                    div {
                        class: "text-red-500 p-4 bg-red-50 rounded",
                        "Error loading tools: {e}"
                    }
                },
                None => rsx! {
                    div {
                        class: "animate-pulse space-y-4",
                        div { class: "h-12 bg-gray-200 rounded" }
                        div { class: "h-12 bg-gray-200 rounded" }
                        div { class: "h-12 bg-gray-200 rounded" }
                    }
                }
            }
        }
    }
}
