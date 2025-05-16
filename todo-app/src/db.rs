use std::sync::{Arc, Mutex};

use crate::models;

pub struct Store(Arc<Mutex<Vec<models::Todo>>>);

impl Store {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
    pub fn add(&self, todo: models::Todo) {
        self.0.lock().unwrap().push(todo);
    }

    pub fn list(&self) -> Vec<models::Todo> {
        self.0.lock().unwrap().clone()
    }

    pub fn find(&self, id: &str) -> Option<models::Todo> {
        self.0
            .lock()
            .unwrap()
            .iter()
            .find(|todo| todo.id == id)
            .cloned()
    }

    pub fn del(&self, id: &str) {
        self.0.lock().unwrap().retain(|todo| todo.id != id);
    }

    pub fn clear(&self) {
        self.0.lock().unwrap().clear();
    }

    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models;

    #[test]
    fn test_add() {
        let store = Store::new();
        store.add(models::Todo {
            id: "1".to_string(),
            title: "test".to_string(),
            is_done: false,
        });
        assert_eq!(store.list().len(), 1);
    }

    #[test]
    fn test_find() {
        let store = Store::new();
        store.add(models::Todo {
            id: "1".to_string(),
            title: "test".to_string(),
            is_done: false,
        });
        assert_eq!(store.find("1").unwrap().title, "test");
    }

    #[test]
    fn test_list() {
        let store = Store::new();
        store.add(models::Todo {
            id: "1".to_string(),
            title: "test".to_string(),
            is_done: false,
        });
        store.add(models::Todo {
            id: "2".to_string(),
            title: "test2".to_string(),
            is_done: false,
        });
        assert_eq!(store.list().len(), 2);
    }

    #[test]
    fn test_del() {
        let store = Store::new();
        store.add(models::Todo {
            id: "1".to_string(),
            title: "test".to_string(),
            is_done: false,
        });
        assert_eq!(store.list().len(), 1);
        store.del("1");
        assert_eq!(store.list().len(), 0);
    }

    #[test]
    fn test_clear() {
        let store = Store::new();
        store.add(models::Todo {
            id: "1".to_string(),
            title: "test".to_string(),
            is_done: false,
        });
        assert_eq!(store.len(), 1);
        store.clear();
        assert_eq!(store.len(), 0);
    }
}
