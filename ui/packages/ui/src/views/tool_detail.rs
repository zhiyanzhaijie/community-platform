use crate::io::get_tool;
use crate::Route;
use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant};

#[component]
pub fn ToolDetail(id: String) -> Element {
    let tool_data = use_server_future(move || get_tool(id.clone()))?;
    let nav = use_navigator();

    rsx! {
        div {
            class: "space-y-6",
            match tool_data() {
                Some(Ok(tool)) => rsx! {
                    div {
                        class: "flex flex-col gap-6",
                        div {
                            class: "flex justify-between items-start",
                            h2 {
                                class: "text-3xl font-bold text-gray-900",
                                "{tool.name}"
                            }
                            span {
                                class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium",
                                "{tool.status}"
                            }
                        }
                        
                        div {
                            class: "prose max-w-none text-gray-600",
                            p { "{tool.description.clone().unwrap_or_default()}" }
                        }

                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-6 bg-white p-6 rounded-lg border border-gray-200 shadow-sm",
                            div {
                                class: "space-y-1",
                                span { class: "text-sm font-medium text-gray-500", "Price" }
                                p { class: "text-xl font-semibold text-gray-900", "{tool.price.amount} {tool.price.currency}" }
                            }
                            div {
                                class: "space-y-1",
                                span { class: "text-sm font-medium text-gray-500", "Category" }
                                p { class: "text-lg text-gray-900", "{tool.category}" }
                            }
                        }

                        div {
                            class: "pt-6 border-t border-gray-200 flex gap-4",
                            Button {
                                variant: ButtonVariant::Outline,
                                on_click: move |_| { nav.push(Route::ToolList {}); },
                                "Back to List"
                            }
                            Button {
                                on_click: move |_| {}, // Placeholder for action
                                "Purchase"
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div {
                        class: "text-red-500 p-4 bg-red-50 rounded border border-red-100",
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
        }
    }
}
