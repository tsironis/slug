use crate::app::Task;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct Metadata {
    date: Option<String>,
    tags: Vec<String>,
    category: String,
    linked_entries: Vec<String>,
}

pub struct Entry {
    metadata: Metadata,
    content: String,
    root_path: PathBuf,
}

pub struct Storage {
    root_path: PathBuf,
    categories: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StorageData {
    tasks: HashMap<NaiveDate, Vec<Task>>,
}

impl StorageData {
    pub fn new(tasks: HashMap<NaiveDate, Vec<Task>>) -> Self {
        Self { tasks }
    }
}

impl Storage {
    pub fn new() -> Self {
        let root_path = match env::var("BULLET_JOURNAL_PATH") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                // Fall back to default: $HOME/.bullet-journal
                let mut path = env::var("HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| env::current_dir().unwrap_or_default());
                path.push(".bullet-journal");
                path
            }
        };

        let categories = vec![
            "future_log",
            "yearly_log",
            "monthly_log",
            "daily",
            "backlog",
        ];

        // Create directory structure
        for category in &categories {
            let mut cat_path = root_path.clone();
            cat_path.push(category);
            fs::create_dir_all(&cat_path).unwrap_or_default();
        }

        Self {
            root_path,
            categories: categories.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn save(&self, data: &StorageData) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.root_path, json)
    }

    pub fn load(&self) -> std::io::Result<StorageData> {
        let data = fs::read_to_string(&self.root_path)?;
        Ok(serde_json::from_str(&data)?)
    }
}
