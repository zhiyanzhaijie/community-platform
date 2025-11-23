use crate::types::MemberDto;
use dioxus::prelude::*;

#[cfg(feature = "web")]
pub async fn save_auth(member: &MemberDto, token: &str) {
    let member_json = serde_json::to_string(member).unwrap_or_default();
    let encoded_member = urlencoding::encode(&member_json);

    let js = format!(
        r#"
        document.cookie = 'token={}; Path=/; SameSite=Lax; Max-Age=86400';
        document.cookie = 'user_info={}; Path=/; SameSite=Lax; Max-Age=86400';
        "#,
        token, encoded_member
    );
    let _ = document::eval(&js);
}

#[cfg(feature = "web")]
pub async fn clear_auth() {
    let js = r#"
        document.cookie = 'token=; Path=/; SameSite=Lax; Max-Age=0';
        document.cookie = 'user_info=; Path=/; SameSite=Lax; Max-Age=0';
    "#;
    let _ = document::eval(js);
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
