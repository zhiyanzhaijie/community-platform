use crate::components::button::{Button, ButtonVariant};
use crate::io::{delete_tool, get_tool};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ToolDetail(id: String) -> Element {
    let id_for_future = id.clone();
    let tool_data = use_server_future(move || get_tool(id_for_future.clone()))?;
    let nav = use_navigator();

    let id_copy = id.clone();
    let handle_delete = move |_: &str| {
        let id = id_copy.clone();
        let nav = nav.clone();
        spawn(async move {
            match delete_tool(id).await {
                Ok(_) => {
                    nav.push(Route::ToolList {});
                }
                Err(e) => {}
            }
        });
    };

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
                            div {
                                class: "flex items-center gap-4",
                                span {
                                    class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium",
                                    "{tool.status}"
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        class: "px-3 py-1 h-auto text-sm hover:bg-gray-50",
                                        onclick: move |_| {
                                            // TODO: Navigate to edit page
                                        },
                                        "Edit"
                                    }
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        class: "px-3 py-1 h-auto text-sm border-red-500 text-red-600 hover:bg-red-50 hover:text-red-700",
                                        onclick: move |_| handle_delete("delete"),
                                        "Delete"
                                    }
                                }
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
                                class: "px-4 py-2 h-auto text-sm hover:bg-gray-50",
                                onclick: move |_| { nav.push(Route::ToolList {}); },
                                "Back to List"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                class: "px-4 py-2 h-auto text-sm",
                                onclick: move |_| {}, // Placeholder for action
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
