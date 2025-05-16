use crate::db::Store;
use crate::models;
use dioxus::prelude::*;

use tokio::sync::OnceCell;

static DB: OnceCell<Store> = OnceCell::const_new();

async fn init_db() -> Store {
    Store::new()
}

async fn get_db() -> &'static Store {
    DB.get_or_init(init_db).await
}

#[server]
pub async fn add_todo(title: String) -> Result<String, ServerFnError> {
    let todo = models::Todo::new(title);
    let id = todo.id.clone();
    let db = get_db().await;
    db.add(todo);
    Ok(id)
}

#[server]
pub async fn delete_todo(id: String) -> Result<(), ServerFnError> {
    let db = get_db().await;
    db.del(&id);
    Ok(())
}

#[server]
pub async fn list_todos() -> Result<Vec<models::Todo>, ServerFnError> {
    let db = get_db().await;
    Ok(db.list())
}

#[server]
pub async fn find_todo(id: String) -> Result<Option<models::Todo>, ServerFnError> {
    let db = get_db().await;
    Ok(db.find(&id))
}
