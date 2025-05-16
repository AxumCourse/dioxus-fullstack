use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::db;
use crate::models;

#[component]
pub fn TodoApp() -> Element {
    let todo_list = use_resource(list_todos);
    rsx! {
        Header { todo_list }
        TodoList { todo_list }
    }
}

#[component]
fn TodoList(todo_list: Resource<Result<Vec<models::Todo>, ServerFnError>>) -> Element {
    match &*todo_list.read_unchecked() {
        Some(Ok(list)) => {
            if list.is_empty() {
                rsx! {
                    div { "暂无代办事项" }
                }
            } else {
                rsx! {
                    ul { class: "space-y-2",
                        for item in list {
                            TodoItem {
                                key: "{item.id}",
                                item: item.clone(),
                                todo_list,
                            }
                        }
                    }
                }
            }
        }

        Some(Err(e)) => rsx! {
            div { "发生错误: {e}" }
        },
        None => rsx! {
            div { "正在加载..." }
        },
    }
}

#[component]
fn TodoItem(
    item: models::Todo,
    todo_list: Resource<Result<Vec<models::Todo>, ServerFnError>>,
) -> Element {
    let item_done = item.clone();
    rsx! {
        li { class: "flex items-center justify-between px-2 py-1 group hover:bg-gray-300 rounded",
            div { class: if item.is_done { "line-through" }, "{item.title}" }
            div { class: "flex items-center justify-end gap-x-1 group-hover:opacity-100 group-hover:visible invisible opacity-0 transition-all",
                if !item.is_done {
                    button {
                        class: "px-1 py-0.5 text-sm bg-green-600 text-white hover:bg-green-700",
                        onclick: move |_| {
                            let id = item_done.id.clone();
                            async move {
                                let _ = done_todo(id).await.unwrap();
                                todo_list.restart();
                            }
                        },
                        "完成"
                    }
                }
                button {
                    class: "px-1 py-0.5 text-sm bg-red-600 text-white hover:bg-red-700",
                    onclick: move |_| {
                        let id = item.id.clone();
                        async move {
                            let _ = delete_todo(id).await.unwrap();
                            todo_list.restart();
                        }
                    },
                    "删除"
                }
            }
        }
    }
}

#[component]
pub fn Header(mut todo_list: Resource<Result<Vec<models::Todo>, ServerFnError>>) -> Element {
    let mut title = use_signal(|| String::new());
    rsx! {
        div { class: "flex justify-between items-center gap-x-2",
            div { class: "grow",
                input {
                    placeholder: "输入代办事项",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                    class: "block w-full px-3 py-1.5 outline-none ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-blue-500",
                }
            }
            div { class: "shrink-0",
                button {
                    class: "px-3 py-1.5 text-white bg-blue-500  hover:bg-blue-600",
                    onclick: move |_| async move {
                        if title().is_empty() {
                            return;
                        }
                        let _ = add_todo(title()).await.unwrap();
                        title.set(String::new());
                        todo_list.restart();
                    },
                    "添加"
                }
            }

        }
    }
}

#[server]
async fn add_todo(title: String) -> Result<String, ServerFnError> {
    let todo = models::Todo::new(title);
    let id = todo.id.clone();
    let db = db::get_db().await;
    db.add(todo);
    Ok(id)
}

#[server]
pub async fn delete_todo(id: String) -> Result<(), ServerFnError> {
    let db = db::get_db().await;
    db.del(&id);
    Ok(())
}

#[server]
pub async fn done_todo(id: String) -> Result<(), ServerFnError> {
    let db = db::get_db().await;
    db.done(&id);
    Ok(())
}

#[server]
pub async fn list_todos() -> Result<Vec<models::Todo>, ServerFnError> {
    let db = db::get_db().await;
    let mut todos = db.list();
    todos.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(todos)
}
