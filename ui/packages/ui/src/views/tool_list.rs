use crate::components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use crate::components::button::{Button, ButtonVariant};
use crate::io::list_tools;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ToolList() -> Element {
    let nav = use_navigator();
    let tools_data = use_server_future(move || list_tools(1, 20))?;

    rsx! {
        div {
            class: "space-y-6",
            div {
                class: "flex justify-between items-center mb-6",
                div {
                    h2 {
                        class: "text-3xl font-bold text-gray-900",
                        "Tools"
                    }
                    p {
                        class: "text-gray-600 mt-1",
                        "Browse our collection of tools using the accordion below:"
                    }
                }
                Button {
                    onclick: move |_| { nav.push(Route::ToolCreate {}); },
                    "Create Tool"
                }
            }

            match tools_data() {
                Some(Ok(response)) => rsx! {
                    Accordion {
                        allow_multiple_open: true,
                        class: "w-full space-y-4", // Ensure full width and spacing
                        for (i, tool) in response.items.into_iter().enumerate() {
                            AccordionItem {
                                index: i,
                                key: "{tool.id}",
                                class: "border border-gray-200 rounded-lg bg-white overflow-hidden",
                                AccordionTrigger {
                                    class: "px-4 py-3 hover:bg-gray-50 transition-colors w-full flex justify-between items-center text-left",
                                    div {
                                        class: "flex items-center gap-3",
                                        span { class: "font-medium text-gray-900", "{tool.name}" }
                                        span { class: "text-sm text-gray-500 bg-gray-100 px-2 py-1 rounded", "{tool.price.amount} {tool.price.currency}" }
                                    }
                                }
                                AccordionContent {
                                    class: "px-4 py-3 bg-gray-50/50 border-t border-gray-100",
                                    div {
                                        class: "flex flex-col gap-3",
                                        div {
                                            class: "text-gray-600",
                                            "{tool.description.clone().unwrap_or_default()}"
                                        }
                                        div {
                                            class: "flex justify-between items-center mt-2",
                                            span { class: "px-2 py-1 bg-blue-50 text-blue-700 rounded text-xs border border-blue-100", "{tool.category}" }
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                class: "text-primary hover:underline text-sm font-medium h-auto p-0",
                                                onclick: move |_| { nav.push(Route::ToolDetail { id: tool.id.clone() }); },
                                                "View Details"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div {
                        class: "text-red-500 p-4 bg-red-50 rounded border border-red-100",
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
