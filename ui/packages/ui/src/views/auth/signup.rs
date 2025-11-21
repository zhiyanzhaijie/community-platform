use crate::io::auth::register;
use crate::types::RegisterRequest;
use crate::Route;
use dioxus::prelude::*;

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
        div {  "signup" }
    }
}
