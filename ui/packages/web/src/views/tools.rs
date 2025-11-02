use dioxus::prelude::*;
use ui::types::Tool;

#[component]
pub fn ToolList() -> Element {
    let tools = use_resource(move || async move {
        // TODO: 实际API调用
        // let response = gloo_net::http::Request::get("http://localhost:8080/api/v1/tools?page=1&page_size=10")
        //     .send()
        //     .await?
        //     .json::<ApiResponse<PaginatedResponse<Tool>>>()
        //     .await?;
        
        // 模拟数据
        Ok::<Vec<Tool>, String>(vec![])
    });

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50",
            
            // 头部
            div { 
                class: "bg-white shadow",
                div { 
                    class: "container mx-auto px-4 py-6 flex justify-between items-center",
                    h1 { 
                        class: "text-3xl font-bold text-gray-900",
                        "工具市场"
                    }
                    Link { 
                        to: "/tools/new",
                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 font-medium",
                        "发布工具"
                    }
                }
            }

            // 内容区
            div { 
                class: "container mx-auto px-4 py-8",
                match tools() {
                    None => rsx! {
                        div { 
                            class: "text-center py-12",
                            "加载中..."
                        }
                    },
                    Some(Ok(tool_list)) => rsx! {
                        if tool_list.is_empty() {
                            div { 
                                class: "text-center py-12 bg-white rounded-lg shadow",
                                p { 
                                    class: "text-gray-500 text-lg",
                                    "暂无工具"
                                }
                                Link { 
                                    to: "/tools/new",
                                    class: "inline-block mt-4 text-blue-600 hover:text-blue-500",
                                    "发布第一个工具"
                                }
                            }
                        } else {
                            div { 
                                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                                for tool in tool_list {
                                    ToolCard { tool: tool.clone() }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div { 
                            class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                            "加载失败: {err}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ToolCard(tool: Tool) -> Element {
    rsx! {
        Link { 
            to: "/tools/{tool.id}",
            class: "block bg-white rounded-lg shadow hover:shadow-lg transition-shadow p-6",
            
            h3 { 
                class: "text-xl font-semibold text-gray-900 mb-2",
                "{tool.name}"
            }
            
            if let Some(desc) = &tool.description {
                p { 
                    class: "text-gray-600 mb-4 line-clamp-2",
                    "{desc}"
                }
            }
            
            div { 
                class: "flex items-center justify-between",
                span { 
                    class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm",
                    "{tool.category}"
                }
                span { 
                    class: "text-lg font-bold text-gray-900",
                    "¥{tool.price.amount}/100"
                }
            }
            
            div { 
                class: "mt-4 text-sm text-gray-500",
                "状态: {tool.status}"
            }
        }
    }
}

#[component]
pub fn ToolDetail(id: String) -> Element {
    let tool = use_resource(move || {
        let _tool_id = id.clone();
        async move {
            // TODO: 实际API调用
            // let response = gloo_net::http::Request::get(&format!("http://localhost:8080/api/v1/tools/{}", tool_id))
            //     .send()
            //     .await?
            //     .json::<ApiResponse<Tool>>()
            //     .await?;
            
            Err::<Tool, String>("未实现".to_string())
        }
    });

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50",
            div { 
                class: "container mx-auto px-4 py-8",
                
                match tool() {
                    None => rsx! {
                        div { class: "text-center py-12", "加载中..." }
                    },
                    Some(Ok(tool_data)) => rsx! {
                        div { 
                            class: "bg-white rounded-lg shadow-lg p-8 max-w-3xl mx-auto",
                            
                            div { 
                                class: "flex justify-between items-start mb-6",
                                h1 { 
                                    class: "text-3xl font-bold text-gray-900",
                                    "{tool_data.name}"
                                }
                                Link { 
                                    to: "/tools/{tool_data.id}/edit",
                                    class: "px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200",
                                    "编辑"
                                }
                            }
                            
                            if let Some(desc) = &tool_data.description {
                                p { 
                                    class: "text-gray-700 text-lg mb-6",
                                    "{desc}"
                                }
                            }
                            
                            div { 
                                class: "grid grid-cols-2 gap-4 mb-6",
                                div {
                                    p { class: "text-sm text-gray-500", "分类" }
                                    p { class: "text-lg font-medium", "{tool_data.category}" }
                                }
                                div {
                                    p { class: "text-sm text-gray-500", "价格" }
                                    p { class: "text-lg font-medium", "¥{tool_data.price.amount}/100 {tool_data.price.currency}" }
                                }
                                div {
                                    p { class: "text-sm text-gray-500", "状态" }
                                    p { class: "text-lg font-medium", "{tool_data.status}" }
                                }
                                div {
                                    p { class: "text-sm text-gray-500", "创建时间" }
                                    p { class: "text-lg font-medium", "{tool_data.created_at}" }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div { 
                            class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                            "加载失败: {err}"
                        }
                    }
                }
            }
        }
    }
}
