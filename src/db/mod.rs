use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::core::{Task, Priority};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseTask {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub priority: i32,
    pub due_date: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub position: i32,
}

pub struct SupabaseClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl SupabaseClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    // Register new user
    pub async fn register(&self, username: &str, email: &str, password: &str) -> Result<SupabaseUser, Box<dyn std::error::Error>> {
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        
        let user = json!({
            "username": username,
            "email": email,
            "password_hash": password_hash,
        });

        let response = self.client
            .post(format!("{}/rest/v1/users", self.base_url))
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&user)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Registration failed: {}", error_text).into());
        }

        let users: Vec<SupabaseUser> = response.json().await?;
        users.into_iter().next().ok_or("No user returned".into())
    }

    // Login user
    pub async fn login(&self, username: &str, password: &str) -> Result<SupabaseUser, Box<dyn std::error::Error>> {
        let response = self.client
            .get(format!("{}/rest/v1/users", self.base_url))
            .header("apikey", &self.api_key)
            .query(&[("username", format!("eq.{}", username))])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("User not found".into());
        }

        let users: Vec<SupabaseUser> = response.json().await?;
        let user = users.into_iter().next().ok_or("User not found")?;

        if bcrypt::verify(password, &user.password_hash)? {
            Ok(user)
        } else {
            Err("Invalid password".into())
        }
    }

    // Create task
    pub async fn create_task(&self, task: &Task, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let supabase_task = json!({
            "id": task.id,
            "user_id": user_id,
            "title": task.title,
            "description": task.description,
            "done": task.done,
            "priority": match task.priority {
                Priority::Low => 1,
                Priority::Medium => 2,
                Priority::High => 3,
            },
            "due_date": task.due_date.map(|d| d.to_rfc3339()),
            "tags": task.tags,
            "created_at": task.created_at.to_rfc3339(),
            "updated_at": task.updated_at.to_rfc3339(),
            "position": task.position,  // Add this
        });

        let response = self.client
            .post(format!("{}/rest/v1/tasks", self.base_url))
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&supabase_task)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to create task: {}", error_text).into());
        }

        Ok(())
    }

    // Get all tasks for user
    pub async fn get_tasks(&self, user_id: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let response = self.client
            .get(format!("{}/rest/v1/tasks", self.base_url))
            .header("apikey", &self.api_key)
            .query(&[("user_id", format!("eq.{}", user_id))])
            .query(&[("order", "position.asc")])  // Changed from created_at.desc
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Failed to fetch tasks".into());
        }

        let supabase_tasks: Vec<SupabaseTask> = response.json().await?;
        
        let tasks = supabase_tasks.into_iter().map(|st| {
            Task {
                id: st.id,
                title: st.title,
                description: st.description,
                done: st.done,
                priority: match st.priority {
                    1 => Priority::Low,
                    3 => Priority::High,
                    _ => Priority::Medium,
                },
                due_date: st.due_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&Utc))),
                tags: st.tags,
                created_at: DateTime::parse_from_rfc3339(&st.created_at).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&st.updated_at).unwrap().with_timezone(&Utc),
                position: st.position,  // Add this
            }
        }).collect();

        Ok(tasks)
    }

    // Update task
    pub async fn update_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        let supabase_task = json!({
            "title": task.title,
            "description": task.description,
            "done": task.done,
            "priority": match task.priority {
                Priority::Low => 1,
                Priority::Medium => 2,
                Priority::High => 3,
            },
            "due_date": task.due_date.map(|d| d.to_rfc3339()),
            "tags": task.tags,
            "updated_at": Utc::now().to_rfc3339(),
            "position": task.position,  // Add this
        });

        let response = self.client
            .patch(format!("{}/rest/v1/tasks", self.base_url))
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .query(&[("id", format!("eq.{}", task.id))])
            .json(&supabase_task)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to update task: {}", error_text).into());
        }

        Ok(())
    }

    //updating position of pre defined tasks
    pub async fn update_positions(&self, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        for task in tasks {
            let update = json!({
                "position": task.position,
                "updated_at": Utc::now().to_rfc3339(),
            });

            self.client
                .patch(format!("{}/rest/v1/tasks", self.base_url))
                .header("apikey", &self.api_key)
                .header("Content-Type", "application/json")
                .query(&[("id", format!("eq.{}", task.id))])
                .json(&update)
                .send()
                .await?;
        }
        Ok(())
    }


    // Delete task
    pub async fn delete_task(&self, task_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .delete(format!("{}/rest/v1/tasks", self.base_url))
            .header("apikey", &self.api_key)
            .query(&[("id", format!("eq.{}", task_id))])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Failed to delete task".into());
        }

        Ok(())
    }

    // Toggle task done status
    pub async fn toggle_done(&self, task_id: &str, current_status: bool) -> Result<(), Box<dyn std::error::Error>> {
        let update = json!({
            "done": !current_status,
            "updated_at": Utc::now().to_rfc3339(),
        });

        let response = self.client
            .patch(format!("{}/rest/v1/tasks", self.base_url))
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .query(&[("id", format!("eq.{}", task_id))])
            .json(&update)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Failed to toggle task".into());
        }

        Ok(())
    }
}