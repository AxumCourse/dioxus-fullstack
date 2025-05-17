use dioxus::prelude::*;

#[component]
pub fn NotFound(path: Vec<String>) -> Element {
    let path = path.join("/");
    rsx! {
        div { "Not Found {path}" }
    }
}
