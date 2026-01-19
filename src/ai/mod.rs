use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::core::{Task, Priority};
use chrono::{Utc, Duration};

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct ParsedTask {
    title: String,
    priority: Option<String>,
    due_days: Option<i64>,
}

pub struct AIAssistant {
    client: Client,
    api_key: String,
}

impl AIAssistant {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn parse_task(&self, input: &str) -> Result<Task, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"Parse this task input and return ONLY a JSON object with these fields:
- title: the main task (string)
- priority: "low", "medium", or "high" (string, optional)
- due_days: days from now for due date (number, optional)

Input: "{}"

Return only valid JSON, no explanation."#,
            input
        );

        match self.call_api(&prompt).await {
            Ok(response) => {
                let parsed = self.parse_json_response(&response)?;
                Ok(parsed)
            }
            Err(_) => {
                Ok(self.offline_parse(input))
            }
        }
    }

    async fn call_api(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = GroqRequest {
            model: "llama-3.3-70b-versatile".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
            max_tokens: 500,
        };

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let groq_response: GroqResponse = response.json().await?;
        
        if let Some(choice) = groq_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from AI".into())
        }
    }

    fn parse_json_response(&self, response: &str) -> Result<Task, Box<dyn std::error::Error>> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        let parsed: ParsedTask = serde_json::from_str(cleaned)?;
        
        let priority = match parsed.priority.as_deref() {
            Some("high") => Priority::High,
            Some("low") => Priority::Low,
            _ => Priority::Medium,
        };

        let due_date = parsed.due_days.map(|days| Utc::now() + Duration::days(days));

        let mut task = Task::new(parsed.title);
        task.priority = priority;
        task.due_date = due_date;

        Ok(task)
    }

    fn offline_parse(&self, input: &str) -> Task {
        let lower = input.to_lowercase();
        
        let priority = if lower.contains("urgent") || lower.contains("high") {
            Priority::High
        } else if lower.contains("low") {
            Priority::Low
        } else {
            Priority::Medium
        };

        let due_date = if lower.contains("today") {
            Some(Utc::now())
        } else if lower.contains("tomorrow") {
            Some(Utc::now() + Duration::days(1))
        } else if lower.contains("week") {
            Some(Utc::now() + Duration::days(7))
        } else {
            None
        };

        let title = input
            .replace("urgent", "")
            .replace("high", "")
            .replace("low", "")
            .replace("today", "")
            .replace("tomorrow", "")
            .trim()
            .to_string();

        let mut task = Task::new(title);
        task.priority = priority;
        task.due_date = due_date;
        task
    }
}