use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};
use crossterm::event::{self, KeyCode};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Clone, Serialize, Deserialize)]
pub enum TaskType {
    Todo,
    Event,
    Note,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub content: String,
    pub task_type: TaskType,
    pub completed: bool,
    pub due_date: Option<DateTime<Local>>,
}

pub enum Mode {
    Normal,  // esc
    Insert,  // i
    Command, // :
    Plan,    // p, check previous day, clean actions, move to next day
    Reflect,
    Future, // r
}

pub enum Log {
    Index,
    Today,
    Weekly,
    Monthly,
    Yearly,
    Future,
    Chapter, // workig vs not working, more of vs less of
}

pub enum Action {
    Task,
    Note,
    Mood,
    Event,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Insert => write!(f, "Insert"),
            Mode::Command => write!(f, "Command"),
            Mode::Future => write!(f, "Future"),
            Mode::Plan => write!(f, "Plan"),
            Mode::Reflect => write!(f, "Reflect"),
        }
    }
}
pub enum Command {
    AddTask(String),
    DeleteTask(usize),
    Toggle(usize),
    Quit,
    Invalid,
}

pub struct App {
    pub mode: Mode,
    pub tasks: HashMap<NaiveDate, Vec<Task>>,
    pub current_date: DateTime<Local>,
    pub input_buffer: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let current_date = Local::now();
        Self {
            mode: Mode::Normal,
            tasks: HashMap::from([(
                NaiveDate::from_ymd_opt(
                    current_date.year(),
                    current_date.month(),
                    current_date.day(),
                )
                .unwrap(),
                vec![Task {
                    content: String::from("lalalal"),
                    task_type: TaskType::Todo,
                    completed: false,
                    due_date: None,
                }],
            )]),
            current_date,
            input_buffer: String::new(),
            should_quit: false,
        }
    }

    pub fn add_task(&mut self, content: String, task_type: TaskType, date: NaiveDate) {
        let task = Task {
            content,
            task_type,
            completed: false,
            due_date: None,
        };
        self.tasks.entry(date).or_insert_with(Vec::new).push(task);
    }

    pub fn toggle_task(&mut self, date: NaiveDate, index: usize) {
        if let Some(tasks) = self.tasks.get_mut(&date) {
            if let Some(task) = tasks.get_mut(index) {
                task.completed = !task.completed;
            }
        }
    }

    pub fn next_week(&mut self) {
        self.current_date = self.current_date + Duration::weeks(1);
    }

    pub fn next_day(&mut self) {
        self.current_date = self.current_date + Duration::days(1);
    }

    pub fn prev_day(&mut self) {
        self.current_date = self.current_date - Duration::days(1);
    }

    pub fn prev_week(&mut self) {
        self.current_date = self.current_date - Duration::weeks(1);
    }

    pub fn parse_command(&self) -> Command {
        let input = self.input_buffer.trim();

        if input.starts_with("add ") {
            Command::AddTask(input[4..].to_string())
        } else if input.starts_with("del ") {
            input[4..]
                .parse()
                .map_or(Command::Invalid, Command::DeleteTask)
        } else if input.starts_with("done ") {
            input[5..].parse().map_or(Command::Invalid, Command::Toggle)
        } else if input == "q" || input == "quit" {
            Command::Quit
        } else {
            Command::Invalid
        }
    }

    pub fn handle_input(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.input_buffer.push(c),
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        let command = self.parse_command();

        let current_date = NaiveDate::from_ymd_opt(
            self.current_date.year(),
            self.current_date.month(),
            self.current_date.day(),
        );
        match command {
            Command::AddTask(content) => {
                if let Some(date) = current_date {
                    self.add_task(content, TaskType::Todo, date);
                }
            }
            Command::DeleteTask(index) => {
                if let Some(date) = current_date {
                    if let Some(tasks) = self.tasks.get_mut(&date) {
                        if index < tasks.len() {
                            tasks.remove(index);
                        }
                    }
                }
            }
            Command::Toggle(index) => {
                if let Some(date) = current_date {
                    self.toggle_task(date, index);
                }
            }
            Command::Quit => self.should_quit = true,
            Command::Invalid => {}
        }
        self.input_buffer.clear();
        self.mode = Mode::Normal;
    }
}
