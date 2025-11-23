use crate::components::{
    button::{Button, ButtonVariant},
    input::Input,
    label::Label,
};
use crate::io::auth::register;
use crate::types::RegisterRequest;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Signup() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let nav = use_navigator();

    let handle_submit = move |e: Event<FormData>| async move {
        e.prevent_default();

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
                println!("Register error: {}", e);
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
                        Label { html_for: "username", class: "mb-1", "Username" }
                        Input {
                            id: "username",
                            value: username(),
                            oninput: move |e: FormEvent| username.set(e.value()),
                            r#type: "text",
                            required: true,
                            placeholder: "johndoe",
                        }
                    }
                    div {
                        Label { html_for: "email", class: "mb-1", "Email" }
                        Input {
                            id: "email",
                            r#type: "email",
                            value: email(),
                            oninput: move |e: FormEvent| email.set(e.value()),
                            required: true,
                            placeholder: "you@example.com",
                        }
                    }
                    div {
                        Label { html_for: "password", class: "mb-1", "Password" }
                        Input {
                            id: "password",
                            r#type: "password",
                            value: password(),
                            oninput: move |e: FormEvent| password.set(e.value()),
                            required: true,
                            placeholder: "••••••••",
                        }
                    }
                    Button {
                        class: "w-full",
                        r#type: "submit",
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
