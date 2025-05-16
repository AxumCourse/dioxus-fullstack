use dioxus::prelude::*;

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


        Echo {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    if input.is_empty() {
        return Ok(String::new());
    }
    let db_ref = db::get().await;
    let (now,): (String,) = sqlx::query_as("SELECT NOW()::TEXT")
        .fetch_one(db_ref)
        .await
        .unwrap();
    Ok(format!("{} ({})", input, now))
}

#[cfg(feature = "server")]
mod db {
    use tokio::sync::OnceCell;

    static DB: OnceCell<sqlx::PgPool> = OnceCell::const_new();

    async fn init() -> sqlx::PgPool {
        dotenv::dotenv().ok();
        let dsn = std::env::var("DATABASE_URL").unwrap();
        sqlx::PgPool::connect(&dsn).await.unwrap()
    }

    pub async fn get() -> &'static sqlx::PgPool {
        DB.get_or_init(init).await
    }
}
