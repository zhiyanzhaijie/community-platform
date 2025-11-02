use dioxus::prelude::*;

use ui::Navbar;
use views::{Login, Register, ToolList, ToolDetail, CreateTool, EditTool};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
        #[route("/")]
        Home {},
        #[route("/tools/new")]
        CreateTool {},
        #[route("/tools/:id/edit")]
        EditTool { id: String },
        #[route("/tools/:id")]
        ToolDetail { id: String },
    #[end_layout]
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Style { 
            r#"
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif; }}
            "#
        }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { ToolList {} }
}

#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                class: "text-gray-700 hover:text-blue-600 font-medium",
                "工具市场"
            }
            Link {
                to: Route::CreateTool {},
                class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 font-medium",
                "发布工具"
            }
            div { class: "ml-auto flex gap-4" }
            Link {
                to: Route::Login {},
                class: "text-gray-700 hover:text-blue-600 font-medium",
                "登录"
            }
            Link {
                to: Route::Register {},
                class: "px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 font-medium",
                "注册"
            }
        }
        Outlet::<Route> {}
    }
}
