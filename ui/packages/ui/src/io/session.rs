use crate::types::MemberDto;
use dioxus::prelude::*;

#[cfg(feature = "web")]
pub async fn save_auth(member: &MemberDto, token: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::window;

    let member_json = serde_json::to_string(member).unwrap_or_default();
    let encoded_member = urlencoding::encode(&member_json);

    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(html_doc) = doc.dyn_ref::<web_sys::HtmlDocument>() {
                let token_cookie = format!("token={}; Path=/; SameSite=Lax; Max-Age=86400", token);
                let user_cookie = format!(
                    "user_info={}; Path=/; SameSite=Lax; Max-Age=86400",
                    encoded_member
                );

                let _ = html_doc.set_cookie(&token_cookie);
                let _ = html_doc.set_cookie(&user_cookie);
            }
        }
    }
}

#[cfg(feature = "web")]
pub async fn clear_auth() {
    use wasm_bindgen::JsCast;
    use web_sys::window;

    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if let Some(html_doc) = doc.dyn_ref::<web_sys::HtmlDocument>() {
                let _ = html_doc.set_cookie("token=; Path=/; SameSite=Lax; Max-Age=0");
                let _ = html_doc.set_cookie("user_info=; Path=/; SameSite=Lax; Max-Age=0");
            }
        }
    }
}

#[cfg(not(feature = "web"))]
pub async fn save_auth(_member: &MemberDto, _token: &str) {
    // TODO: Implement for desktop/mobile using file storage or keychain
    println!("Saving auth not implemented for non-web targets yet");
}

#[cfg(not(feature = "web"))]
pub async fn clear_auth() {
    // TODO: Implement for desktop/mobile
    println!("Clearing auth not implemented for non-web targets yet");
}
