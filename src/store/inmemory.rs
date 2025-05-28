// use chrono::{DateTime, Local};
use dashmap::DashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub date: String,
    pub time: String,
    pub message_type: String,
    pub title: String,
    pub text: String,
}

pub struct Store {
    data: Arc<DashMap<String, Event>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Event> {
        self.data.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: Event) {
        self.data.insert(key, value);
    }

    #[cfg(test)]
    pub fn remove(&self, key: &str) {
        self.data.remove(key);
    }

    pub fn load_from_file(&self, path: &PathBuf) {
        let path = Path::new(path);
        let mut file = std::fs::File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() == 5 {
                let event = Event {
                    date: parts[0].to_string(),
                    time: parts[1].to_string(),
                    message_type: parts[2].to_ascii_lowercase(),
                    title: parts[3].trim().to_string(),
                    text: parts[4].trim().to_string(),
                };
                self.set(event.time.clone(), event);
            }
        }
    }
}
