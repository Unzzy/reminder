mod store;
mod utils;

use std::env;
use std::path::PathBuf;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), String> {
    let interval_sec = 30u64; // Количество секунд в интервале проверки событий
    let args = env::args().collect::<Vec<String>>();
    let path = if args.len() > 1 {
        let input_path = PathBuf::from(&args[1]);
        if input_path.is_absolute() {
            input_path
        } else {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            current_dir.join(input_path)
        }
    } else {
        utils::utils::get_resource_path("scheduler.csv")
    };
    let store = store::inmemory::Store::new();
    store.load_from_file(&path);
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            _ = utils::utils::check_scheduled_events(&store, interval_sec) => {}
        }
    }
    println!("Выполнение программы завершено корректно");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::inmemory::Event;

    #[tokio::test]
    async fn test_store() {
        let store = store::inmemory::Store::new();
        let event = Event {
            date: "test_date".to_string(),
            time: "21:00".to_string(),
            title: "test title".to_string(),
            text: "test text".to_string(),
        };

        let test_event = Event {
            date: "test_date".to_string(),
            time: "21:00".to_string(),
            title: "test title".to_string(),
            text: "test text".to_string(),
        };
        store.set("key".to_string(), event);
        assert_eq!(store.get("key").unwrap(), test_event);
        store.remove("key");
        assert!(store.get("key").is_none());
    }

    #[tokio::test]
    async fn test_create_store_from_file() {
        let path = utils::utils::get_resource_path("scheduler.csv");
        let st = store::inmemory::Store::new();
        st.load_from_file(&path);
        let result = st.get("21:42").unwrap();
        let text = result.text;
        assert_eq!(text, "Test!");
    }
}
