use crate::components::{
    button::{Button, ButtonVariant},
    input::Input,
    label::Label,
};
use crate::io::tool::create_tool;
use crate::types::CreateToolRequest;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ToolCreate() -> Element {
    let nav = use_navigator();
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut category = use_signal(|| "".to_string());
    let mut price_amount = use_signal(|| "".to_string());

    let handle_submit = move |e: Event<FormData>| async move {
        e.prevent_default();

        let amount = match price_amount().parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid price: must be a valid number");
                return;
            }
        };

        let req = CreateToolRequest {
            name: name(),
            description: Some(description()),
            category: category(),
            price_amount: amount,
            price_currency: "CNY".to_string(),
        };

        match create_tool(req).await {
            Ok(_) => {
                nav.push(Route::ToolList {});
            }
            Err(e) => {
                println!("Error creating tool: {}", e);
            }
        }
    };

    rsx! {
        div {
            class: "max-w-2xl mx-auto bg-white p-8 rounded-lg shadow-sm border border-gray-200",
            div {
                class: "mb-8",
                h1 { class: "text-3xl font-bold text-gray-900", "Create New Tool" }
                p { class: "text-gray-600 mt-2", "Fill in the details below to create a new tool." }
            }

            form {
                onsubmit: handle_submit,
                class: "space-y-6",

                div {
                    Label { html_for: "name", class: "mb-1", "Name" }
                    Input {
                        id: "name",
                        value: name(),
                        oninput: move |e: FormEvent| name.set(e.value()),
                        r#type: "text",
                        required: true,
                        placeholder: "e.g. Image Optimizer",
                    }
                }

                div {
                    Label { html_for: "description", class: "mb-1", "Description" }
                    textarea {
                        id: "description",
                        class: "flex min-h-[80px] w-full rounded border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                        value: description(),
                        oninput: move |e| description.set(e.value()),
                        placeholder: "Describe your tool...",
                        rows: "4",
                    }
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div {
                        Label { html_for: "category", class: "mb-1", "Category" }
                        Input {
                            id: "category",
                            value: category(),
                            oninput: move |e: FormEvent| category.set(e.value()),
                            r#type: "text",
                            required: true,
                            placeholder: "e.g. Utility",
                        }
                    }
                    div {
                        Label { html_for: "price", class: "mb-1", "Price (CNY)" }
                        Input {
                            id: "price",
                            value: price_amount(),
                            oninput: move |e: FormEvent| price_amount.set(e.value()),
                            r#type: "number",
                            required: true,
                            placeholder: "0",
                        }
                    }
                }

                div {
                    class: "flex justify-end gap-4",
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| { nav.push(Route::ToolList {}); },
                        "Cancel"
                    }
                    Button {
                        r#type: "submit",
                        "Create Tool"
                    }
                }
            }
        }
    }
}
