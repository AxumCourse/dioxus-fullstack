use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;
use image_hosting::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let admin_token = use_persistent("ADMIN_TOKEN", || String::new());
    use_context_provider(|| admin_token);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: MAIN_CSS }

        Router::<Route> {}
    }
}
