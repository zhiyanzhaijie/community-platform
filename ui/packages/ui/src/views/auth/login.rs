use crate::io::auth::login;
use crate::types::LoginRequest;
use crate::Route;
use dioxus::prelude::*;
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;
use lumen_blocks::components::button::Button;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    let handle_submit = move |e: Event<FormData>| async move {
        e.prevent_default();
        error_msg.set(None);

        let req = LoginRequest {
            email: email(),
            password: password(),
        };

        match login(req).await {
            Ok(resp) => {
                // Set Cookie on Client Side via JS eval
                // This allows subsequent SSR requests to carry the token
                let js = format!(
                    "document.cookie = 'token={}; Path=/; SameSite=Lax; Max-Age=86400';",
                    resp.token
                );
                // Dioxus 0.7 document::eval
                let _ = document::eval(&js);

                nav.push(Route::ToolList {});
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
                h2 { class: "text-2xl font-bold mb-6 text-center text-gray-900", "Log in to your account" }
                form {
                    onsubmit: handle_submit,
                    class: "space-y-5",
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
