use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "text-xl text-red-600", "home" }
    }
}
