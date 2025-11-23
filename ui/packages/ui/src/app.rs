use crate::{
    components::{
        avatar::{Avatar, AvatarFallback},
        button::{Button, ButtonVariant},
        icons::Logo,
        separator::Separator,
    },
    io::auth::get_current_member,
    types::MemberDto,
    views::{Home, Login, NotFound, Signup, ToolCreate, ToolDetail, ToolList},
};
use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENT_STYLE: Asset = asset!("/assets/dx-components-theme.css");
#[derive(Clone, Copy)]
pub struct UserContext(pub Resource<Result<Option<MemberDto>, ServerFnError>>);
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[nest("/tools")]
    #[route("/")]
    ToolList {},
    #[route("/new")]
    ToolCreate {},
    #[route("/:id")]
    ToolDetail { id: String },
    #[end_nest]
    #[nest("/auth")]
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[end_nest]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
#[component]
pub fn App() -> Element {
    if let Ok(user_resource) = use_server_future(move || get_current_member()) {
        use_context_provider(|| UserContext(user_resource));
    }

    rsx! {
        document::Link {
            rel: "icon",
            href: FAVICON,
        }
        document::Stylesheet {
            href: TAILWIND,
        }
        document::Stylesheet {
            href: DX_COMPONENT_STYLE,
        }
        Router::<Route> {


        }
    }
}
#[component]
fn Layout() -> Element {
    let mut is_collapsed = use_signal(|| false);
    let user_context = use_context::<UserContext>();
    let user_resource = user_context.0;

    let current_user = match user_resource() {
        Some(Ok(Some(member))) => Some(member),
        _ => None,
    };

    let width_class = if is_collapsed() { "w-16" } else { "w-64" };
    let margin_class = if is_collapsed() {
        "md:ml-16"
    } else {
        "md:ml-64"
    };
    rsx! {
        div {
            class: "min-h-screen flex bg-gray-50 flex-col md:flex-row",
            div {
                class: "md:hidden h-16 bg-white border-b border-gray-200 flex items-center px-4 justify-between sticky top-0 z-40",
                Logo {

                }
                div {
                    class: "w-10",
                }
            }
            aside {
                class: "hidden md:flex bg-white border-r border-gray-200 flex-shrink-0 fixed inset-y-0 left-0 z-50 flex-col transition-all duration-300 {width_class}",
                // Simple sidebar toggle button
                div {
                    class: "absolute -right-3 top-20 z-[100]",
                    button {
                        class: "rounded-full w-6 h-6 bg-white shadow-sm border border-gray-200 hover:bg-gray-50 flex items-center justify-center text-xs p-0 cursor-pointer",
                        onclick: move |_| {
                            web_sys::console::log_1(&format!("Sidebar toggle clicked. Current state: {}", is_collapsed()).into());
                            is_collapsed.set(!is_collapsed());
                        },
                        if is_collapsed() {
                            ">" // ▶
                        } else {
                            "<" // ◀
                        }
                    }
                }
                Navigation {
                    _is_mobile: false,
                    is_collapsed: is_collapsed(),
                    user: current_user,
                }
            }
            // Main Content
            main {
                class: "flex-1 min-w-0 transition-all duration-300 {margin_class}",
                div {
                    class: "max-w-7xl mx-auto p-4 md:p-8",
                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            div {
                                class: "flex items-center justify-center h-full",
                                "Loading..."
                            }
                        },
                        Outlet::<Route> {


                        }
                    }
                }
            }
        }
    }
}
#[component]
fn Navigation(_is_mobile: bool, is_collapsed: bool, user: Option<MemberDto>) -> Element {
    let nav = use_navigator();
    let header_class = if is_collapsed {
        "justify-center px-2"
    } else {
        ""
    };
    let justify_class = if is_collapsed {
        "justify-center"
    } else {
        "justify-start"
    };
    let tools_label = "Tools Plaza".to_string();
    rsx! {
        div {
            class: "flex flex-col h-full overflow-hidden",
            div {
                class: "h-16 flex items-center px-6 flex-shrink-0 {header_class}",
                div {
                    class: "text-xl text-red-700 font-bold cursor-pointer truncate",
                    onclick: move |_| {
                        nav.push(Route::Home {});
                    },
                    Logo {
                        class: "w-full",
                    }
                }
            }
            Separator {


            }
            // 2. Features Navigation
            nav {
                class: "flex-1 px-2 py-6 space-y-2 overflow-y-auto overflow-x-hidden",
                if !is_collapsed {
                    div {
                        class: "text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 px-2",
                        "Features"
                    }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    class: "justify-start w-full",
                    onclick: move |_| {
                        nav.push(Route::ToolList {});
                    },
                    if is_collapsed {
                        span {
                            class: "flex items-center justify-center w-full",
                            title: "{tools_label}",
                            "{tools_label.chars().next().unwrap_or('?')}"
                        }
                    } else {
                        span {
                            "{tools_label}"
                        }
                    }
                }
            }
            // 3. Placeholder simple auth section (no dropdown / avatar)
            Separator {


            }
            div {
                class: "p-4 mt-auto flex items-center justify-between text-sm text-gray-600",
                if let Some(u) = user {
                    div {
                        class: "flex items-center gap-2",
                        Avatar {
                            AvatarFallback {
                                "{u.username.chars().next().unwrap_or('?')}"
                            }
                        }
                        if !is_collapsed {
                            span {
                                "{u.username}"
                            }
                        }
                    }
                } else {
                    if !is_collapsed {
                        div {
                            class: "w-full flex gap-2",
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "flex-1 h-8 text-xs px-3",
                                onclick: move |_| {
                                    nav.push(Route::Login {});
                                },
                                "Log in"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "flex-1 h-8 text-xs px-3",
                                onclick: move |_| {
                                    nav.push(Route::Signup {});
                                },
                                "Sign up"
                            }
                        }
                    } else {
                        // Collapsed state auth buttons
                        div {
                            class: "w-full flex flex-col gap-2",
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "w-full h-8 text-xs px-0 flex items-center justify-center",
                                onclick: move |_| {
                                    nav.push(Route::Login {});
                                },
                                "L"
                            }
                        }
                    }
                }
            }
        }
    }
}
