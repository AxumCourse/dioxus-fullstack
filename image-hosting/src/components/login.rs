use dioxus::prelude::*;

#[component]
pub fn AdminLogin() -> Element {
    let mut token = use_context::<Signal<String>>();
    let mut password = use_signal(|| String::new());
    let mut has_err = use_signal(|| false);
    rsx! {
        div { class: "fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white/70 rounded p-6 shadow space-y-4 lg:min-w-96",
            div { "请输入密码进行登录" }
            div {
                input {
                    class: "block w-full px-3 py-1.5 outline-none ring-1 ring-inset rounded focus:ring-indigo-600",
                    class: if has_err() { "ring-red-300" } else { "ring-gray-300 " },
                    r#type: "password",
                    oninput: move |e| {
                        password.set(e.value());
                        has_err.set(false);
                    },
                }
            }
            div { class: "flex justify-end",
                button {
                    class: "px-3 py-1.5 bg-gray-900 text-gray-50 rounded text-sm hover:bg-gray-800",
                    onclick: move |_| async move {
                        if password().is_empty() {
                            has_err.set(true);
                            return;
                        }
                        match admin_login_server(password()).await {
                            Ok(t) => {
                                token.set(t);
                            }
                            Err(_) => has_err.set(true),
                        }
                    },
                    "登录"
                }
            }
        }
    }
}

#[server]
async fn admin_login_server(password: String) -> Result<String, ServerFnError> {
    if password != "foobar" {
        return Err(ServerFnError::ServerError("密码错误".into()));
    }
    Ok("foobar".into())
}
