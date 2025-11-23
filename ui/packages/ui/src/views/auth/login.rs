use crate::components::{
    button::{Button, ButtonVariant},
    input::Input,
    label::Label,
};
use crate::io::auth::login;
use crate::io::session;
use crate::types::LoginRequest;
use crate::app::UserContext;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let nav = use_navigator();
    let mut user_context = use_context::<UserContext>();

    let handle_submit = move |e: Event<FormData>| async move {
        e.prevent_default();

        let req = LoginRequest {
            email: email(),
            password: password(),
        };

        match login(req).await {
            Ok(resp) => {
                // Use session abstraction to save auth (Token + User Info)
                session::save_auth(&resp.member, &resp.token).await;

                // Restart resource to refresh global state
                user_context.0.restart();

                nav.push(Route::ToolList {});
            }
            Err(e) => {
                println!("Login error: {}", e);
            }
        }
    };

    rsx! {
        div {
            class: "min-h-[80vh] flex items-center justify-center",
            div {
                class: "max-w-md w-full bg-white p-8 rounded-lg shadow-md border border-gray-100",
                h2 { class: "text-2xl font-bold mb-6 text-center text-gray-900", "Log in to your account" }
                form {
                    onsubmit: handle_submit,
                    class: "space-y-5",
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
                        "Log in"
                    }
                }
                div {
                    class: "mt-6 text-center text-sm text-gray-600",
                    "Don't have an account? "
                    Link {
                        to: Route::Signup {},
                        class: "font-medium text-primary hover:underline",
                        "Sign up"
                    }
                }
            }
        }
    }
}
