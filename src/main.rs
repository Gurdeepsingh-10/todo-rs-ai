mod ui;
mod core;
mod ai;
mod config;
mod db;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use ui::{AppState, Mode};
use core::Priority;
use dotenv::dotenv;
use ai::AIAssistant;
use db::SupabaseClient;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting TODO AI application");
    
    dotenv().ok();
    
    let api_key = std::env::var("GROQ_API_KEY").unwrap_or_default();
    let ai = AIAssistant::new(api_key);
    
    // Initialize Supabase
    let supabase_url = std::env::var("SUPABASE_URL")
        .expect("SUPABASE_URL must be set in .env");
    let supabase_key = std::env::var("SUPABASE_KEY")
        .expect("SUPABASE_KEY must be set in .env");
    
    let supabase = SupabaseClient::new(supabase_url, supabase_key);
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.supabase = Some(supabase);
    state.mode = Mode::Login;

    let mut g_pressed = false;

    loop {
        terminal.draw(|f| {
            ui::render(f, &state);
        })?;

        if let Event::Key(key) = event::read()? {
            match state.mode {
                Mode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => state.next(),
                        KeyCode::Char('k') | KeyCode::Up => state.previous(),
                        KeyCode::Char(' ') => {
                            if let (Some(task), Some(ref sb), Some(ref user)) = 
                                (state.tasks.get(state.selected), &state.supabase, &state.current_user) {
                                match sb.toggle_done(&task.id, task.done).await {
                                    Ok(_) => {
                                        if let Ok(tasks) = sb.get_tasks(&user.id).await {
                                            state.tasks = tasks;
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to toggle task: {}", e);
                                    }
                                }
                            }
                        }
                        KeyCode::Char('?') => state.show_help = !state.show_help,
                        KeyCode::Char(':') => {
                            state.mode = Mode::Command;
                            state.command_input.clear();
                        }
                        KeyCode::Char('e') => {
                            if let Some(task) = state.tasks.get(state.selected) {
                                state.mode = Mode::Edit;
                                state.command_input = task.title.clone();
                                state.editing_task = Some(task.id.clone());
                            }
                        }
                        KeyCode::Char('1') => {
                            if let (Some(task), Some(ref sb), Some(ref user)) = 
                                (state.tasks.get_mut(state.selected), &state.supabase, &state.current_user) {
                                task.priority = Priority::Low;
                                if let Ok(_) = sb.update_task(task).await {
                                    if let Ok(tasks) = sb.get_tasks(&user.id).await {
                                        state.tasks = tasks;
                                    }
                                }
                            }
                        }
                        KeyCode::Char('2') => {
                            if let (Some(task), Some(ref sb), Some(ref user)) = 
                                (state.tasks.get_mut(state.selected), &state.supabase, &state.current_user) {
                                task.priority = Priority::Medium;
                                if let Ok(_) = sb.update_task(task).await {
                                    if let Ok(tasks) = sb.get_tasks(&user.id).await {
                                        state.tasks = tasks;
                                    }
                                }
                            }
                        }
                        KeyCode::Char('3') => {
                            if let (Some(task), Some(ref sb), Some(ref user)) = 
                                (state.tasks.get_mut(state.selected), &state.supabase, &state.current_user) {
                                task.priority = Priority::High;
                                if let Ok(_) = sb.update_task(task).await {
                                    if let Ok(tasks) = sb.get_tasks(&user.id).await {
                                        state.tasks = tasks;
                                    }
                                }
                            }
                        }
                        KeyCode::Char('g') => {
                            if g_pressed {
                                state.selected = 0;
                                g_pressed = false;
                            } else {
                                g_pressed = true;
                            }
                        }
                        KeyCode::Char('G') => {
                            if !state.tasks.is_empty() {
                                state.selected = state.tasks.len() - 1;
                            }
                        }
                        KeyCode::Char('d') => {
                            if let (Some(task), Some(ref sb), Some(ref user)) = 
                                (state.tasks.get(state.selected), &state.supabase, &state.current_user) {
                                let task_id = task.id.clone();
                                match sb.delete_task(&task_id).await {
                                    Ok(_) => {
                                        if let Ok(tasks) = sb.get_tasks(&user.id).await {
                                            state.tasks = tasks;
                                            if state.selected >= state.tasks.len() && state.selected > 0 {
                                                state.selected -= 1;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to delete task: {}", e);
                                    }
                                }
                            }
                        }
                        _ => {
                            g_pressed = false;
                        }
                    }
                }
                Mode::Command => {
                    match key.code {
                        KeyCode::Esc => {
                            state.mode = Mode::Normal;
                            state.command_input.clear();
                            state.command_history.reset();
                        }
                        KeyCode::Enter => {
                            state.command_history.add(state.command_input.clone());
                            handle_command(&mut state, &ai).await?;
                            state.mode = Mode::Normal;
                            state.command_history.reset();
                        }
                        KeyCode::Up => {
                            if let Some(cmd) = state.command_history.previous() {
                                state.command_input = cmd;
                            }
                        }
                        KeyCode::Down => {
                            if let Some(cmd) = state.command_history.next() {
                                state.command_input = cmd;
                            } else {
                                state.command_input.clear();
                            }
                        }
                        KeyCode::Char(c) => {
                            state.command_input.push(c);
                        }
                        KeyCode::Backspace => {
                            state.command_input.pop();
                        }
                        _ => {}
                    }
                }
                Mode::Edit => {
                    match key.code {
                        KeyCode::Esc => {
                            state.mode = Mode::Normal;
                            state.command_input.clear();
                            state.editing_task = None;
                        }
                        KeyCode::Enter => {
                            if let (Some(task_id), Some(ref sb), Some(ref user)) = 
                                (&state.editing_task, &state.supabase, &state.current_user) {
                                if let Some(task) = state.tasks.iter_mut().find(|t| &t.id == task_id) {
                                    task.title = state.command_input.clone();
                                    let _ = sb.update_task(task).await;
                                    if let Ok(tasks) = sb.get_tasks(&user.id).await {
                                        state.tasks = tasks;
                                    }
                                }
                            }
                            state.mode = Mode::Normal;
                            state.command_input.clear();
                            state.editing_task = None;
                        }
                        KeyCode::Char(c) => {
                            state.command_input.push(c);
                        }
                        KeyCode::Backspace => {
                            state.command_input.pop();
                        }
                        _ => {}
                    }
                }
                Mode::Login => {
                    match key.code {
                        KeyCode::Esc => {
                            std::process::exit(0);
                        }
                        KeyCode::Enter => {
                            let parts: Vec<&str> = state.command_input.split_whitespace().collect();
                            if parts.len() == 2 {
                                let username = parts[0];
                                let password = parts[1];
                                
                                if let Some(ref sb) = state.supabase {
                                    match sb.login(username, password).await {
                                        Ok(user) => {
                                            info!("User logged in: {}", user.username);
                                            let user_id = user.id.clone();
                                            state.current_user = Some(user);
                                            state.mode = Mode::Normal;
                                            
                                            if let Some(ref sb) = state.supabase {
                                                match sb.get_tasks(&user_id).await {
                                                    Ok(tasks) => state.tasks = tasks,
                                                    Err(e) => {
                                                        error!("Failed to load tasks: {}", e);
                                                        state.status_message = Some("Failed to load tasks".to_string());
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            state.command_input = "Login failed. Press : to register".to_string();
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Char(':') => {
                            if state.command_input.is_empty() {
                                state.mode = Mode::Register;
                                state.command_input.clear();
                            } else {
                                state.command_input.push(':');
                            }
                        }
                        KeyCode::Char(c) => {
                            state.command_input.push(c);
                        }
                        KeyCode::Backspace => {
                            state.command_input.pop();
                        }
                        _ => {}
                    }
                }
                Mode::Register => {
                    match key.code {
                        KeyCode::Esc => {
                            state.mode = Mode::Login;
                            state.command_input.clear();
                        }
                        KeyCode::Enter => {
                            let parts: Vec<&str> = state.command_input.split_whitespace().collect();
                            if parts.len() == 3 {
                                let username = parts[0];
                                let password = parts[1];
                                let email = parts[2];
                                
                                if let Some(ref sb) = state.supabase {
                                    match sb.register(username, email, password).await {
                                        Ok(user) => {
                                            info!("User registered: {}", user.username);
                                            state.current_user = Some(user);
                                            state.mode = Mode::Normal;
                                        }
                                        Err(e) => {
                                            state.command_input = format!("Registration failed: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Char(c) => {
                            state.command_input.push(c);
                        }
                        KeyCode::Backspace => {
                            state.command_input.pop();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

async fn handle_command(state: &mut AppState, ai: &AIAssistant) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = state.command_input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        "add" => {
            if parts.len() > 1 {
                let input = parts[1..].join(" ");
                let task = ai.parse_task(&input).await?;
                
                info!("Task created: {}", task.title);
                
                if let (Some(ref sb), Some(ref user)) = (&state.supabase, &state.current_user) {
                    sb.create_task(&task, &user.id).await?;
                    state.tasks = sb.get_tasks(&user.id).await?;
                    state.status_message = Some("Task added!".to_string());
                }
            }
        }
        "done" => {
            if let (Some(task), Some(ref sb), Some(ref user)) = 
                (state.tasks.get(state.selected), &state.supabase, &state.current_user) {
                sb.toggle_done(&task.id, task.done).await?;
                state.tasks = sb.get_tasks(&user.id).await?;
            }
        }
        "sync" => {
            if let (Some(ref sb), Some(ref user)) = (&state.supabase, &state.current_user) {
                state.tasks = sb.get_tasks(&user.id).await?;
                state.status_message = Some("Tasks synced!".to_string());
            }
        }
        "quit" | "q" => {
            std::process::exit(0);
        }
        "config" => {
            if parts.len() == 1 {
                state.status_message = Some(format!("Config: {:?}", dirs::home_dir().unwrap_or_default()));
            } else if parts.len() == 3 {
                let action = parts[1];
                let key = parts[2];
                state.config.keybindings.insert(action.to_string(), key.to_string());
                let _ = state.config.save();
                state.status_message = Some(format!("Keybinding updated: {} -> {}", action, key));
            }
        }
        _ => {}
    }
    
    state.command_input.clear();
    Ok(())
}