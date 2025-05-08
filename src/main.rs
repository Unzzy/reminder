mod store;
mod utils;

use tokio::signal;
use utils::*;

#[tokio::main]
async fn main() -> Result<(), String> {
    let title = "Training";
    let interval_sec = 60u64;

    let path = utils::get_resource_path("scheduler.txt");
    let store = store::inmemory::Store::new();
    store.load_from_file(&path);
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            _ = check_scheduled_events(&store, title, interval_sec) => {}
        }
    }
    println!("Выполнение программы завершено корректно");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store() {
        let store = store::inmemory::Store::new();
        store.set("key".to_string(), "value".to_string());
        assert_eq!(store.get("key").unwrap(), "value");
        store.remove("key");
        assert!(store.get("key").is_none());
    }

    #[tokio::test]
    async fn test_create_store_from_file() {
        let path = utils::get_resource_path("scheduler.txt");
        let st = store::inmemory::Store::new();
        st.load_from_file(&path);
        let result = st.get("21:42").unwrap();
        assert_eq!(result, "Test!");
    }
}
