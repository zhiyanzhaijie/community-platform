use crate::components::button::{Button, ButtonVariant};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            h2 {
                class: "text-4xl font-bold text-gray-900",
                "Welcome to Dioxus Router Demo"
            }
            p {
                class: "text-lg text-gray-600 mb-6",
                "This demo showcases Dioxus components and routing capabilities."
            }

            Tabs {
                default_value: "features",
                class: "w-full",
                TabList {
                    class: "flex border-b border-gray-200 mb-4",
                    TabTrigger {
                        value: "features",
                        index: 0usize,
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 border-b-2 border-transparent data-[state=active]:border-blue-600 data-[state=active]:text-blue-600",
                        "Features"
                    }
                    TabTrigger {
                        value: "routing",
                        index: 1usize,
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 border-b-2 border-transparent data-[state=active]:border-blue-600 data-[state=active]:text-blue-600",
                        "Routing"
                    }
                    TabTrigger {
                        value: "components",
                        index: 2usize,
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 border-b-2 border-transparent data-[state=active]:border-blue-600 data-[state=active]:text-blue-600",
                        "Components"
                    }
                }

                TabContent {
                    value: "features",
                    index: 0usize,
                    class: "space-y-4",
                    h3 { class: "text-xl font-semibold text-gray-900", "Key Features" }
                    ul {
                        class: "list-disc list-inside space-y-2 text-gray-600",
                        li { "Cross-platform UI framework built in Rust" }
                        li { "Reactive state management with signals" }
                        li { "Component-based architecture" }
                        li { "Type-safe routing" }
                    }
                }

                TabContent {
                    value: "routing",
                    index: 1usize,
                    class: "space-y-4",
                    h3 { class: "text-xl font-semibold text-gray-900 mb-4", "Try these routes:" }
                    ul {
                        class: "space-y-3",
                        li {
                            Link {
                                to: Route::ToolList {},
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "w-full justify-start",
                                    "/tool - Tool List"
                                }
                            }
                        }
                    }
                }

                TabContent {
                    value: "components",
                    index: 2usize,
                    class: "space-y-4",
                    h3 { class: "text-xl font-semibold text-gray-900", "Dioxus Components Used" }
                    ul {
                        class: "list-disc list-inside space-y-2 text-gray-600",
                        li { "Button - Interactive buttons with variants" }
                        li { "Separator - Visual dividers" }
                        li { "Tabs - Tabbed interface (you're using it now!)" }
                        li { "Accordion - Collapsible content sections" }
                    }
                }
            }
        }
    }
}
