use crate::io::auth::login;
use crate::types::LoginRequest;
use crate::Route;
use dioxus::prelude::*;

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
        }
    }
}
