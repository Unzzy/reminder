use dashmap::DashMap;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

pub struct Store {
    data: Arc<DashMap<String, String>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn remove(&self, key: &str) {
        self.data.remove(key);
    }
    pub fn load_from_file(&self, path: &str) {
        let path = Path::new(path);
        println!("{}", path.display());
        let mut file = std::fs::File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        for line in contents.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                self.set(parts[0].to_string(), parts[1].to_string());
            }
        }
    }
}
