use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub keybindings: HashMap<String, String>,
    pub theme: Theme,
    pub ai_settings: AISettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISettings {
    pub auto_parse: bool,
    pub offline_fallback: bool,
}

impl Default for Config {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        
        // Navigation
        keybindings.insert("move_down".to_string(), "j".to_string());
        keybindings.insert("move_up".to_string(), "k".to_string());
        keybindings.insert("goto_top".to_string(), "gg".to_string());
        keybindings.insert("goto_bottom".to_string(), "G".to_string());
        
        // Actions
        keybindings.insert("toggle_done".to_string(), "space".to_string());
        keybindings.insert("delete_task".to_string(), "d".to_string());
        keybindings.insert("edit_task".to_string(), "e".to_string());
        keybindings.insert("priority_low".to_string(), "1".to_string());
        keybindings.insert("priority_medium".to_string(), "2".to_string());
        keybindings.insert("priority_high".to_string(), "3".to_string());
        
        // Mode
        keybindings.insert("command_mode".to_string(), ":".to_string());
        keybindings.insert("help".to_string(), "?".to_string());
        keybindings.insert("quit".to_string(), "q".to_string());

        Self {
            keybindings,
            theme: Theme {
                primary_color: "Cyan".to_string(),
                secondary_color: "Yellow".to_string(),
                accent_color: "Green".to_string(),
            },
            ai_settings: AISettings {
                auto_parse: true,
                offline_fallback: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        
        if let Ok(contents) = fs::read_to_string(&config_path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            let default = Self::default();
            let _ = default.save();
            default
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, json)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".config");
        path.push("todo-ai");
        path.push("config.json");
        path
    }

    pub fn get_key(&self, action: &str) -> Option<String> {
        self.keybindings.get(action).cloned()
    }
}

pub struct CommandHistory {
    history: Vec<String>,
    current_index: Option<usize>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: None,
        }
    }

    pub fn add(&mut self, command: String) {
        if !command.is_empty() && (self.history.is_empty() || self.history.last() != Some(&command)) {
            self.history.push(command);
        }
        self.current_index = None;
    }

    pub fn previous(&mut self) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }

        let index = match self.current_index {
            Some(i) if i > 0 => i - 1,
            None => self.history.len() - 1,
            _ => return None,
        };

        self.current_index = Some(index);
        self.history.get(index).cloned()
    }

    pub fn next(&mut self) -> Option<String> {
        let index = match self.current_index {
            Some(i) if i < self.history.len() - 1 => i + 1,
            _ => return None,
        };

        self.current_index = Some(index);
        self.history.get(index).cloned()
    }

    pub fn reset(&mut self) {
        self.current_index = None;
    }
}

pub struct CommandPalette {
    commands: Vec<Command>,
}

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
}

impl CommandPalette {
    pub fn new() -> Self {
        let commands = vec![
            Command {
                name: "add".to_string(),
                description: "Add a new task".to_string(),
                usage: ":add <task description>".to_string(),
            },
            Command {
                name: "done".to_string(),
                description: "Mark selected task as done".to_string(),
                usage: ":done".to_string(),
            },
            Command {
                name: "sync".to_string(),
                description: "Sync tasks with server".to_string(),
                usage: ":sync".to_string(),
            },
            Command {
                name: "quit".to_string(),
                description: "Quit the application".to_string(),
                usage: ":quit or :q".to_string(),
            },
        ];

        Self { commands }
    }

    pub fn search(&self, query: &str) -> Vec<Command> {
        if query.is_empty() {
            return self.commands.clone();
        }

        self.commands
            .iter()
            .filter(|cmd| cmd.name.starts_with(query) || cmd.description.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect()
    }

    pub fn get_all(&self) -> Vec<Command> {
        self.commands.clone()
    }
}