use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use crate::core::Task;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub struct AuthManager {
    pool: SqlitePool,
}

impl AuthManager {
    pub async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                email TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_tasks (
                user_id TEXT NOT NULL,
                task_id TEXT NOT NULL,
                synced_at TEXT NOT NULL,
                PRIMARY KEY (user_id, task_id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn register(&self, username: &str, password: &str, email: &str) -> Result<User, Box<dyn std::error::Error>> {
        let password_hash = hash(password, DEFAULT_COST)?;
        let user_id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, email, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(&user_id)
        .bind(username)
        .bind(&password_hash)
        .bind(email)
        .bind(&created_at)
        .execute(&self.pool)
        .await?;

        Ok(User {
            id: user_id,
            username: username.to_string(),
            password_hash,
            email: email.to_string(),
            created_at,
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<User, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            r#"
            SELECT id, username, password_hash, email, created_at
            FROM users
            WHERE username = ?1
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        let password_hash: String = row.get("password_hash");
        
        if verify(password, &password_hash)? {
            Ok(User {
                id: row.get("id"),
                username: row.get("username"),
                password_hash,
                email: row.get("email"),
                created_at: row.get("created_at"),
            })
        } else {
            Err("Invalid password".into())
        }
    }

    pub async fn link_task_to_user(&self, user_id: &str, task_id: &str) -> Result<(), sqlx::Error> {
        let synced_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO user_tasks (user_id, task_id, synced_at)
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(user_id)
        .bind(task_id)
        .bind(synced_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_tasks(&self, user_id: &str) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT task_id FROM user_tasks WHERE user_id = ?1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| r.get("task_id")).collect())
    }
}

pub struct SyncManager {
    auth: AuthManager,
}

impl SyncManager {
    pub async fn new(pool: SqlitePool) -> Result<Self, Box<dyn std::error::Error>> {
        let auth = AuthManager::new(pool).await?;
        Ok(Self { auth })
    }

    pub async fn register(&self, username: &str, password: &str, email: &str) -> Result<User, Box<dyn std::error::Error>> {
        self.auth.register(username, password, email).await
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<User, Box<dyn std::error::Error>> {
        self.auth.login(username, password).await
    }

    pub async fn sync_tasks(&self, user_id: &str, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        for task in tasks {
            self.auth.link_task_to_user(user_id, &task.id).await?;
        }
        Ok(())
    }

    pub async fn get_user_task_ids(&self, user_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(self.auth.get_user_tasks(user_id).await?)
    }
}