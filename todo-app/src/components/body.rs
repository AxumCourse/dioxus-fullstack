use dioxus::prelude::*;

#[component]
pub fn Body() -> Element {
    rsx! {
        TodoList {}
    }
}

#[component]
fn TodoList() -> Element {
    rsx! {
        ul { class: "space-y-2",
            for i in 0..10 {
                TodoItem { key: "{i}" }
            }
        }
    }
}

#[component]
fn TodoItem() -> Element {
    rsx! {
        li { class: "flex items-center justify-between px-2 py-1 group hover:bg-gray-300 rounded",
            div { "TodoItem" }
            div { class: "flex items-center justify-end gap-x-1 group-hover:opacity-100 group-hover:visible invisible opacity-0 transition-all",
                button { class: "px-1 py-0.5 text-sm bg-green-600 text-white hover:bg-green-700",
                    "完成"
                }
                button { class: "px-1 py-0.5 text-sm bg-red-600 text-white hover:bg-red-700",
                    "删除"
                }
            }
        }
    }
}
