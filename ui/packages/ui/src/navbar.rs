use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav {
            class: "bg-white shadow-md",
            div {
                class: "container mx-auto px-4 py-4 flex items-center gap-6",
                {children}
            }
        }
    }
}
