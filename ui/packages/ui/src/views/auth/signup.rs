use crate::io::auth::register;
use crate::types::RegisterRequest;
use crate::Route;
use dioxus::prelude::*;
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;
use lumen_blocks::components::button::Button;

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
            }
            Err(e) => {
                error_msg.set(Some(e.to_string()));
            }
        }
    };

    rsx! {
        div {
            class: "min-h-[80vh] flex items-center justify-center",
            div {
                class: "max-w-md w-full bg-white p-8 rounded-lg shadow-md border border-gray-100",
                h2 { class: "text-2xl font-bold mb-6 text-center text-gray-900", "Create an account" }
                form {
                    onsubmit: handle_submit,
                    class: "space-y-5",
                    div {
                        Label { "Username" }
                        Input {
                            full_width: true,
                            value: username(),
                            on_change: move |e: FormEvent| username.set(e.value()),
                            required: true,
                            placeholder: "johndoe",
                        }
                    }
                    div {
                        Label { "Email" }
                        Input {
                            input_type: "email",
                            full_width: true,
                            value: email(),
                            on_change: move |e: FormEvent| email.set(e.value()),
                            required: true,
                            placeholder: "you@example.com",
                        }
                    }
                    div {
                        Label { "Password" }
                        Input {
                            input_type: "password",
                            full_width: true,
                            value: password(),
                            on_change: move |e: FormEvent| password.set(e.value()),
                            required: true,
                            placeholder: "••••••••",
                        }
                    }
                    if let Some(msg) = error_msg() {
                        div { class: "text-red-500 text-sm p-2 bg-red-50 rounded", "{msg}" }
                    }
                    Button {
                        button_type: "submit",
                        full_width: true,
                        "Sign up"
                    }
                }
                div {
                    class: "mt-6 text-center text-sm text-gray-600",
                    "Already have an account? "
                    Link {
                        to: Route::Login {},
                        class: "font-medium text-primary hover:underline",
                        "Log in"
                    }
                }
            }
        }
    }
}
