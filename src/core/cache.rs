use std::collections::HashMap;
use crate::core::Task;


#[allow(dead_code)]
pub struct TaskCache {
    cache: HashMap<String, Task>,
}


#[allow(dead_code)]
impl TaskCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, id: &str) -> Option<&Task> {
        self.cache.get(id)
    }

    pub fn insert(&mut self, task: Task) {
        self.cache.insert(task.id.clone(), task);
    }

    pub fn remove(&mut self, id: &str) {
        self.cache.remove(id);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn update_all(&mut self, tasks: Vec<Task>) {
        self.cache.clear();
        for task in tasks {
            self.cache.insert(task.id.clone(), task);
        }
    }
}