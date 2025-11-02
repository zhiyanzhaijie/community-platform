use dioxus::prelude::*;
use ui::types::{CreateToolRequest, UpdateToolRequest};

#[component]
pub fn CreateTool() -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut category = use_signal(|| String::new());
    let mut price_amount = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let _nav = navigator();

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        spawn(async move {
            loading.set(true);
            error.set(None);

            let price = match price_amount().parse::<i64>() {
                Ok(p) => p * 100, // 转换为分
                Err(_) => {
                    error.set(Some("价格格式错误".to_string()));
                    loading.set(false);
                    return;
                }
            };

            let _request = CreateToolRequest {
                name: name(),
                description: if description().is_empty() { None } else { Some(description()) },
                category: category(),
                price_amount: price,
                price_currency: "CNY".to_string(),
            };

            // TODO: 实际API调用
            // let response = gloo_net::http::Request::post("http://localhost:8080/api/v1/tools")
            //     .json(&request)
            //     .unwrap()
            //     .send()
            //     .await;

            loading.set(false);
            // nav.push("/tools");
        });
    };

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50 py-8",
            div { 
                class: "container mx-auto px-4",
                div { 
                    class: "max-w-2xl mx-auto",
                    
                    h1 { 
                        class: "text-3xl font-bold text-gray-900 mb-8",
                        "发布新工具"
                    }

                    form { 
                        class: "bg-white rounded-lg shadow p-8 space-y-6",
                        onsubmit: handle_submit,

                        if let Some(err) = error() {
                            div { 
                                class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                                "{err}"
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "工具名称 *"
                            }
                            input { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "text",
                                required: true,
                                value: "{name}",
                                oninput: move |evt| name.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "描述"
                            }
                            textarea { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                rows: 4,
                                value: "{description}",
                                oninput: move |evt| description.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "分类 *"
                            }
                            input { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "text",
                                required: true,
                                placeholder: "例如: 开发工具、设计工具",
                                value: "{category}",
                                oninput: move |evt| category.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "价格 (元) *"
                            }
                            input { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "number",
                                required: true,
                                min: 0,
                                step: "0.01",
                                value: "{price_amount}",
                                oninput: move |evt| price_amount.set(evt.value()),
                            }
                        }

                        div { 
                            class: "flex gap-4",
                            button { 
                                class: "flex-1 py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50",
                                r#type: "submit",
                                disabled: loading(),
                                if loading() { "发布中..." } else { "发布工具" }
                            }
                            Link { 
                                to: "/tools",
                                class: "flex-1 py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 text-center",
                                "取消"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EditTool(id: String) -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut category = use_signal(|| String::new());
    let mut price_amount = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut loaded = use_signal(|| false);
    let _nav = navigator();

    // 先克隆所有需要的id副本
    let resource_id = id.clone();
    let submit_id = id.clone();

    // 加载工具数据
    let _ = use_resource(move || {
        let _tool_id_clone = resource_id.clone();
        async move {
            if !loaded() {
                // TODO: 实际API调用加载工具数据
                // let response = gloo_net::http::Request::get(&format!("http://localhost:8080/api/v1/tools/{}", tool_id_clone))
                //     .send()
                //     .await?
                //     .json::<ApiResponse<Tool>>()
                //     .await?;
                
                // 填充表单数据
                loaded.set(true);
            }
            Ok::<(), String>(())
        }
    });
    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        let _submit_tool_id = submit_id.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);

            let price = if !price_amount().is_empty() {
                match price_amount().parse::<i64>() {
                    Ok(p) => Some(p * 100),
                    Err(_) => {
                        error.set(Some("价格格式错误".to_string()));
                        loading.set(false);
                        return;
                    }
                }
            } else {
                None
            };

            let _request = UpdateToolRequest {
                name: if !name().is_empty() { Some(name()) } else { None },
                description: if !description().is_empty() { Some(description()) } else { None },
                category: if !category().is_empty() { Some(category()) } else { None },
                price_amount: price,
                price_currency: price.map(|_| "CNY".to_string()),
            };

            // TODO: 实际API调用
            // let response = gloo_net::http::Request::put(&format!("http://localhost:8080/api/v1/tools/{}", submit_tool_id))
            //     .json(&request)
            //     .unwrap()
            //     .send()
            //     .await;

            loading.set(false);
            // nav.push(&format!("/tools/{}", submit_tool_id));
        });
    };

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50 py-8",
            div { 
                class: "container mx-auto px-4",
                div { 
                    class: "max-w-2xl mx-auto",
                    
                    h1 { 
                        class: "text-3xl font-bold text-gray-900 mb-8",
                        "编辑工具"
                    }

                    form { 
                        class: "bg-white rounded-lg shadow p-8 space-y-6",
                        onsubmit: handle_submit,

                        if let Some(err) = error() {
                            div { 
                                class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                                "{err}"
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "工具名称"
                            }
                            input { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "text",
                                value: "{name}",
                                oninput: move |evt| name.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "描述"
                            }
                            textarea { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                rows: 4,
                                value: "{description}",
                                oninput: move |evt| description.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "分类"
                            }
                            input { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "text",
                                value: "{category}",
                                oninput: move |evt| category.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "价格 (元)"
                            }
                            input { 
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "number",
                                min: 0,
                                step: "0.01",
                                value: "{price_amount}",
                                oninput: move |evt| price_amount.set(evt.value()),
                            }
                        }

                        div { 
                            class: "flex gap-4",
                            button { 
                                class: "flex-1 py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50",
                                r#type: "submit",
                                disabled: loading(),
                                if loading() { "保存中..." } else { "保存修改" }
                            }
                            Link { 
                                to: "/tools/{id}",
                                class: "flex-1 py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 text-center",
                                "取消"
                            }
                        }
                    }
                }
            }
        }
    }
}
