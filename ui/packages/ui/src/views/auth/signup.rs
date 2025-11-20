use dioxus::prelude::*;
use crate::components::button::{Button, ButtonVariant};
use crate::Route;
use crate::types::RegisterRequest;
use crate::io::auth::register;

#[component]
pub fn Signup() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    let handle_submit = move |e: Event<FormData>| async move {
        e.prevent_default();
        error_msg.set(None);
        
        let req = RegisterRequest {
            username: username(),
            email: email(),
            password: password(),
        };
        
        match register(req).await {
            Ok(_) => {
                // Registration successful, navigate to login
                nav.push(Route::Login {});
            },
            Err(e) => {
                error_msg.set(Some(e.to_string()));
            }
        }
    };

    rsx! {
        div {
            class: "min-h-[calc(100vh-4rem)] flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8",
            div {
                class: "max-w-md w-full space-y-8",
                div {
                    h2 {
                        class: "mt-6 text-center text-3xl font-extrabold text-gray-900",
                        "Create your account"
                    }
                    p {
                        class: "mt-2 text-center text-sm text-gray-600",
                        "Or "
                        Link {
                            to: Route::Login {},
                            class: "font-medium text-blue-600 hover:text-blue-500",
                            "sign in to existing account"
                        }
                    }
                }

                if let Some(msg) = error_msg() {
                    div {
                        class: "bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded relative",
                        "{msg}"
                    }
                }

                form {
                    class: "mt-8 space-y-6",
                    onsubmit: handle_submit,
                    div {
                        class: "rounded-md shadow-sm -space-y-px",
                        div {
                            input {
                                class: "appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm",
                                placeholder: "Username",
                                r#type: "text",
                                required: true,
                                value: "{username}",
                                oninput: move |e| username.set(e.value())
                            }
                        }
                        div {
                            input {
                                class: "appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm",
                                placeholder: "Email address",
                                r#type: "email",
                                required: true,
                                value: "{email}",
                                oninput: move |e| email.set(e.value())
                            }
                        }
                        div {
                            input {
                                class: "appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm",
                                placeholder: "Password",
                                r#type: "password",
                                required: true,
                                value: "{password}",
                                oninput: move |e| password.set(e.value())
                            }
                        }
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        class: "w-full justify-center",
                        r#type: "submit",
                        "Sign up"
                    }
                }
            }
        }
    }
}
