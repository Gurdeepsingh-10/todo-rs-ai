use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::core::{Task, Priority};

use crate::config::{Config, CommandHistory};
use std::time::{Instant, Duration};


pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected: usize,
    pub mode: Mode,
    pub command_input: String,
    pub show_help: bool,
    pub editing_task: Option<String>,
    pub current_user: Option<crate::db::SupabaseUser>,  // Changed
    pub config: Config,
    pub command_history: CommandHistory,
    pub status_message: Option<String>,
    pub supabase: Option<crate::db::SupabaseClient>,   // Changed
    pub status_timer: Option<Instant>
}

pub enum Mode {
    Normal,
    Command,
    Edit,
    Login,      // Add this
    Register,
}


impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            selected: 0,
            mode: Mode::Normal,
            command_input: String::new(),
            show_help: false,
            editing_task: None,
            current_user: None,
            config: Config::load(),
            command_history: CommandHistory::new(),
            status_message: None,
            supabase: None,
            status_timer: None,
        }
    }

    pub fn next(&mut self) {
        if !self.tasks.is_empty() {
            self.selected = (self.selected + 1) % self.tasks.len();
        }
    }
    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_timer = Some(Instant::now());
    }
    pub fn clear_old_status(&mut self) {
        if let Some(timer) = self.status_timer {
            if timer.elapsed() > Duration::from_secs(2) {
                self.status_message = None;
                self.status_timer = None;
            }
        }
    }

    pub fn previous(&mut self) {
        if !self.tasks.is_empty() {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.tasks.len() - 1;
            }
        }
    }
}

pub fn render(f: &mut Frame, state: &AppState) {
    if state.show_help {
        render_help(f);
        return;
    }

    // Show login prompt if not logged in
    if state.current_user.is_none() && matches!(state.mode, Mode::Login | Mode::Register) {
        render_auth(f, state);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let header = Paragraph::new("TODO AI - Terminal Task Manager")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let status = if task.done { "[✓]" } else { "[ ]" };
            let priority = match task.priority {
                Priority::Low => "P1",
                Priority::Medium => "P2",
                Priority::High => "P3",
            };
            let content = format!("{} {} {}", status, priority, task.title);
            
            let style = if i == state.selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if task.done {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Tasks"));
    f.render_widget(list, chunks[1]);

    let status_text = if let Some(msg) = &state.status_message {
        msg.clone()
    } else {
        match state.mode {
            Mode::Normal => {
                "q: quit | j/k: navigate | Shift+J/K: reorder | space: toggle | :: command | ?: help".to_string()
            }
            Mode::Command => format!(":{}", state.command_input),
            Mode::Edit => format!("Edit: {}", state.command_input),
            Mode::Login | Mode::Register => state.command_input.clone(),
        }
    };


    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn render_help(f: &mut Frame) {
    let help_text = vec![
        Line::from(vec![Span::styled("KEYBOARD SHORTCUTS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![Span::styled("Navigation:", Style::default().fg(Color::Yellow))]),
        Line::from("  j / ↓         - Move down"),
        Line::from("  k / ↑         - Move up"),
        Line::from("  gg            - Go to top"),
        Line::from("  G             - Go to bottom"),
        Line::from(""),
        Line::from(vec![Span::styled("Reorder Tasks:", Style::default().fg(Color::Yellow))]),
        Line::from("  Shift+J       - Move task down in list"),
        Line::from("  Shift+K       - Move task up in list"),
        Line::from(""),
        Line::from(vec![Span::styled("Actions:", Style::default().fg(Color::Yellow))]),
        Line::from("  space         - Toggle task done/undone"),
        Line::from("  d             - Delete task"),
        Line::from("  e             - Edit task"),
        Line::from("  1/2/3         - Set priority (Low/Medium/High)"),
        Line::from("  ?             - Toggle this help screen"),
        Line::from("  q             - Quit application"),
        Line::from(""),
        Line::from(vec![Span::styled("Command Mode (press :)", Style::default().fg(Color::Yellow))]),
        Line::from("  :add <task>              - Add new task"),
        Line::from("                             Example: :add Buy groceries"),
        Line::from(""),
        Line::from("  :done                    - Mark selected task as done"),
        Line::from(""),
        Line::from("  :sync                    - Sync tasks from cloud"),
        Line::from(""),
        Line::from("  :quit or :q              - Quit application"),
        Line::from(""),
        Line::from(vec![Span::styled("Command Mode Controls:", Style::default().fg(Color::Yellow))]),
        Line::from("  Esc           - Exit command mode"),
        Line::from("  Enter         - Execute command"),
        Line::from("  Backspace     - Delete character"),
        Line::from("  ↑/↓           - Navigate command history"),
        Line::from(""),
        Line::from(vec![Span::styled("Press ? to close this help", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))]),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help & Commands"))
        .alignment(Alignment::Left);
    
    let area = f.size();
    f.render_widget(help , area);
}

fn render_auth(f: &mut Frame, state: &AppState) {
    let (title, instruction) = match state.mode {
        Mode::Login => ("Login", "Format: username password | Press : to register"),
        Mode::Register => ("Register", "Format: username password email | Press Esc to go back"),
        _ => ("Auth", ""),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let title_widget = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(title_widget, chunks[0]);

    let instruction_widget = Paragraph::new(instruction)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    
    f.render_widget(instruction_widget, chunks[1]);

    let input_widget = Paragraph::new(format!("> {}", state.command_input))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Input"));
    
    f.render_widget(input_widget, chunks[2]);
}