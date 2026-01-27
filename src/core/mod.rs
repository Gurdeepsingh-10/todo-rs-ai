use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;

#[derive(Debug)]
pub enum TaskError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            TaskError::NotFound(msg) => write!(f, "Not found: {}", msg),
            TaskError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Task {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: String::new(),
            done: false,
            priority: Priority::Medium,
            due_date: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

pub mod cache;
pub use cache::TaskCache;