use crate::components::button::{Button, ButtonVariant};
use crate::io::get_tool;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ToolDetail(id: String) -> Element {
    let tool_data = use_server_future(move || get_tool(id.clone()))?;

    rsx! {
        div {
            class: "space-y-6",
            match tool_data() {
                Some(Ok(tool)) => rsx! {
                    h2 {
                        class: "text-3xl font-bold text-gray-900",
                        "{tool.name}"
                    }
                    p {
                        class: "text-lg text-gray-600 leading-relaxed",
                        "{tool.description.clone().unwrap_or_default()}"
                    }
                    div {
                        class: "mt-4 p-6 bg-white border border-gray-200 rounded-lg shadow-sm",
                        h3 { class: "text-lg font-semibold mb-2", "Details" }
                        div { class: "grid grid-cols-2 gap-4",
                            div {
                                p { class: "text-sm text-gray-500", "Price" }
                                p { class: "font-medium", "{tool.price.amount} {tool.price.currency}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Category" }
                                p { class: "font-medium", "{tool.category}" }
                            }
                            div {
                                p { class: "text-sm text-gray-500", "Status" }
                                p { class: "font-medium", "{tool.status}" }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div {
                        class: "text-red-500 p-4 bg-red-50 rounded",
                        "Error loading tool details: {e}"
                    }
                },
                None => rsx! {
                    div {
                        class: "animate-pulse space-y-4",
                        div { class: "h-8 w-1/3 bg-gray-200 rounded" }
                        div { class: "h-24 bg-gray-200 rounded" }
                    }
                }
            }

            div {
                class: "mt-8 pt-8 border-t border-gray-200",
                Link {
                    to: Route::ToolList {},
                    Button {
                        variant: ButtonVariant::Outline,
                        "← Back to Tools"
                    }
                }
            }
        }
    }
}
