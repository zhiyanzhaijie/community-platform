use dioxus::prelude::*;
use crate::io::list_tools;
use crate::components::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use crate::components::button::{Button, ButtonVariant};
use crate::Route;

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
                        class: "space-y-4",
                        collapsible: true,
                        for (index, tool) in response.items.iter().enumerate() {
                            AccordionItem {
                                index,
                                class: "bg-white border border-gray-200 rounded-lg overflow-hidden",
                                AccordionTrigger {
                                    class: "w-full px-6 py-4 text-left hover:bg-gray-50 transition-colors",
                                    div {
                                        class: "flex items-center justify-between",
                                        div {
                                            h3 {
                                                class: "text-xl font-semibold text-gray-900",
                                                "{tool.name}"
                                            }
                                            p {
                                                class: "text-sm text-gray-600 mt-1",
                                                "{tool.category}"
                                            }
                                        }
                                    }
                                }
                                AccordionContent {
                                    class: "px-6 py-4 border-t border-gray-200 bg-gray-50",
                                    p {
                                        class: "text-gray-700 mb-4",
                                        "{tool.description.clone().unwrap_or_default()}"
                                    }
                                    Link {
                                        to: Route::ToolDetail { id: tool.id.clone() },
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            class: "mt-2",
                                            "View Details →"
                                        }
                                    }
                                }
                            }
                        }
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
