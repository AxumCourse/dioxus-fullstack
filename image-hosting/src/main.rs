use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;
use image_hosting::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();

    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());

    #[cfg(feature = "web")]
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

#[cfg(feature = "server")]
async fn launch_server() {
    let addr = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    dioxus::logger::tracing::info!("listening on {addr}");

    let app = axum::Router::new().serve_dioxus_application(ServeConfigBuilder::new(), App);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
