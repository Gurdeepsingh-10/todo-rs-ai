mod ui;
mod core;
mod ai;
mod sync;
mod config;

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
use core::{TaskManager, Priority};
use dotenv::dotenv;
use ai::AIAssistant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let api_key = std::env::var("GROQ_API_KEY").unwrap_or_default();
    let ai = AIAssistant::new(api_key);
    
    let db_path = std::env::current_dir()?.join("tasks.db");
    let db_url = format!("sqlite://{}", db_path.display());
    let task_manager = TaskManager::new(&db_url).await?;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.task_manager = Some(task_manager);
    
    if let Some(ref tm) = state.task_manager {
        state.tasks = tm.get_all_tasks().await?;
    }

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
                            if let Some(task) = state.tasks.get(state.selected) {
                                if let Some(ref tm) = state.task_manager {
                                    tm.toggle_done(&task.id).await?;
                                    state.tasks = tm.get_all_tasks().await?;
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
                            if let Some(task) = state.tasks.get_mut(state.selected) {
                                task.priority = Priority::Low;
                                if let Some(ref tm) = state.task_manager {
                                    tm.update_task(task).await?;
                                    state.tasks = tm.get_all_tasks().await?;
                                }
                            }
                        }
                        KeyCode::Char('2') => {
                            if let Some(task) = state.tasks.get_mut(state.selected) {
                                task.priority = Priority::Medium;
                                if let Some(ref tm) = state.task_manager {
                                    tm.update_task(task).await?;
                                    state.tasks = tm.get_all_tasks().await?;
                                }
                            }
                        }
                        KeyCode::Char('3') => {
                            if let Some(task) = state.tasks.get_mut(state.selected) {
                                task.priority = Priority::High;
                                if let Some(ref tm) = state.task_manager {
                                    tm.update_task(task).await?;
                                    state.tasks = tm.get_all_tasks().await?;
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
                            if let Some(task) = state.tasks.get(state.selected) {
                                if let Some(ref tm) = state.task_manager {
                                    tm.delete_task(&task.id).await?;
                                    state.tasks = tm.get_all_tasks().await?;
                                    if state.selected >= state.tasks.len() && state.selected > 0 {
                                        state.selected -= 1;
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
                        }
                        KeyCode::Enter => {
                            handle_command(&mut state, &ai).await?;
                            state.mode = Mode::Normal;
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
                            if let Some(task_id) = &state.editing_task {
                                if let Some(ref tm) = state.task_manager {
                                    if let Some(task) = state.tasks.iter_mut().find(|t| &t.id == task_id) {
                                        task.title = state.command_input.clone();
                                        let _ = tm.update_task(task).await;
                                        state.tasks = tm.get_all_tasks().await?;
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
                
                if let Some(ref tm) = state.task_manager {
                    tm.create_task(&task).await?;
                    state.tasks = tm.get_all_tasks().await?;
                }
            }
        }
        "done" => {
            if let Some(task) = state.tasks.get(state.selected) {
                if let Some(ref tm) = state.task_manager {
                    tm.toggle_done(&task.id).await?;
                    state.tasks = tm.get_all_tasks().await?;
                }
            }
        }
        "quit" | "q" => {
            std::process::exit(0);
        }
        _ => {}
    }
    
    state.command_input.clear();
    Ok(())
}