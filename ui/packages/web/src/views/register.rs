use dioxus::prelude::*;
use ui::types::RegisterRequest;

#[component]
pub fn Register() -> Element {
    let mut email = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let _nav = navigator();

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        spawn(async move {
            loading.set(true);
            error.set(None);

            let _request = RegisterRequest {
                email: email(),
                username: username(),
                password: password(),
            };

            // TODO: 实际API调用
            // let response = gloo_net::http::Request::post("http://localhost:8080/api/v1/members/register")
            //     .json(&request)
            //     .unwrap()
            //     .send()
            //     .await;

            // 模拟成功
            loading.set(false);
            // nav.push("/login");
        });
    };

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50 flex items-center justify-center py-12 px-4",
            div { 
                class: "max-w-md w-full space-y-8",
                div {
                    h2 { 
                        class: "text-center text-3xl font-bold text-gray-900",
                        "注册账号"
                    }
                }

                form { 
                    class: "mt-8 space-y-6 bg-white p-8 rounded-lg shadow",
                    onsubmit: handle_submit,

                    if let Some(err) = error() {
                        div { 
                            class: "bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded",
                            "{err}"
                        }
                    }

                    div { 
                        class: "space-y-4",
                        
                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700",
                                "邮箱"
                            }
                            input { 
                                class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "email",
                                required: true,
                                value: "{email}",
                                oninput: move |evt| email.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700",
                                "用户名"
                            }
                            input { 
                                class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "text",
                                required: true,
                                value: "{username}",
                                oninput: move |evt| username.set(evt.value()),
                            }
                        }

                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700",
                                "密码"
                            }
                            input { 
                                class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                r#type: "password",
                                required: true,
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                        }
                    }

                    button { 
                        class: "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50",
                        r#type: "submit",
                        disabled: loading(),
                        if loading() { "注册中..." } else { "注册" }
                    }

                    div { 
                        class: "text-center text-sm",
                        "已有账号？ "
                        Link { 
                            to: "/login",
                            class: "text-blue-600 hover:text-blue-500",
                            "登录"
                        }
                    }
                }
            }
        }
    }
}
