use crate::io::list_tools;
use crate::Route;
use dioxus::prelude::*;
use lumen_blocks::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

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
                Some(Ok(response)) => rsx! {
                    Accordion {
                        class: "border border-gray-200 rounded-lg bg-white",
                        for (i, tool) in response.items.into_iter().enumerate() {
                            AccordionItem {
                                key: "{tool.id}",
                                index: i,
                                AccordionTrigger {
                                    div {
                                        class: "flex justify-between w-full items-center pr-4",
                                        span { class: "font-medium text-gray-900", "{tool.name}" }
                                        span { class: "text-sm text-gray-500 bg-gray-100 px-2 py-1 rounded", "{tool.price.amount} {tool.price.currency}" }
                                    }
                                }
                                AccordionContent {
                                    div {
                                        class: "text-gray-600 mb-4 pt-2",
                                        "{tool.description.clone().unwrap_or_default()}"
                                    }
                                    div {
                                        class: "flex justify-between items-center mt-2",
                                        span { class: "px-2 py-1 bg-blue-50 text-blue-700 rounded text-xs border border-blue-100", "{tool.category}" }
                                        Link {
                                            to: Route::ToolDetail { id: tool.id },
                                            class: "text-primary hover:underline text-sm font-medium",
                                            "View Details"
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
