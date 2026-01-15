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
use core::{Task, TaskManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = std::env::current_dir()?.join("tasks.db");
    let db_url = format!("sqlite:{}", db_path.display());
    let task_manager = TaskManager::new(&db_url).await?;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.task_manager = Some(task_manager);
    
    // Load tasks
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
                            handle_command(&mut state).await?;
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
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

async fn handle_command(state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = state.command_input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        "add" => {
            if parts.len() > 1 {
                let title = parts[1..].join(" ");
                let task = Task::new(title);
                
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