use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

pub struct TaskManager {
    pool: SqlitePool,
}

impl TaskManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                done INTEGER NOT NULL,
                priority INTEGER NOT NULL,
                due_date TEXT,
                tags TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn create_task(&self, task: &Task) -> Result<(), sqlx::Error> {
        let tags_json = serde_json::to_string(&task.tags).unwrap();
        let due_date = task.due_date.map(|d| d.to_rfc3339());

        sqlx::query(
            r#"
            INSERT INTO tasks (id, title, description, done, priority, due_date, tags, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.done as i32)
        .bind(task.priority.clone() as i32)
        .bind(due_date)
        .bind(tags_json)
        .bind(task.created_at.to_rfc3339())
        .bind(task.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM tasks ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let tasks = rows
            .iter()
            .map(|row| {
                let tags_json: String = row.get("tags");
                let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
                
                let due_date_str: Option<String> = row.get("due_date");
                let due_date = due_date_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc)));

                let priority_int: i32 = row.get("priority");
                let priority = match priority_int {
                    1 => Priority::Low,
                    3 => Priority::High,
                    _ => Priority::Medium,
                };

                Task {
                    id: row.get("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    done: row.get::<i32, _>("done") != 0,
                    priority,
                    due_date,
                    tags,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&Utc),
                }
            })
            .collect();

        Ok(tasks)
    }

    pub async fn update_task(&self, task: &Task) -> Result<(), sqlx::Error> {
        let tags_json = serde_json::to_string(&task.tags).unwrap();
        let due_date = task.due_date.map(|d| d.to_rfc3339());

        sqlx::query(
            r#"
            UPDATE tasks 
            SET title = ?1, description = ?2, done = ?3, priority = ?4, 
                due_date = ?5, tags = ?6, updated_at = ?7
            WHERE id = ?8
            "#,
        )
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.done as i32)
        .bind(task.priority.clone() as i32)
        .bind(due_date)
        .bind(tags_json)
        .bind(Utc::now().to_rfc3339())
        .bind(&task.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_task(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tasks WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn toggle_done(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE tasks 
            SET done = NOT done, updated_at = ?1
            WHERE id = ?2
            "#,
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}