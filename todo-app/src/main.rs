use dioxus::prelude::*;
use todo_app::components::TodoApp;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: MAIN_CSS }

        div { class: "container mx-auto max-w-lg shadow-lg p-3 space-y-3", TodoApp {} }
    }
}
