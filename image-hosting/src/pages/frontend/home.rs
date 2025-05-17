use dioxus::prelude::*;

use crate::components::Upload;

#[component]
pub fn Home() -> Element {
    rsx! {
        Upload {}
    }
}
