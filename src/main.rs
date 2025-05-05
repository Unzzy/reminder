mod store;

#[tokio::main]
async fn main() {
    let store = store::inmemory::Store::new();
    store.set("key".to_string(), "value".to_string());
    println!("Value: {}", store.get("key").unwrap());
    store.remove("key");
    println!("Value: {:?}", store.get("key"));
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
}

#[tokio::test]
async fn test_create_store_from_file() {
    let st = store::inmemory::Store::new();
    st.load_from_file("E:/repo/reminder/resources/scheduler.txt");
    let result = st.get("some event").unwrap();
    assert_eq!(result, "2025-05-05 12:00:00")
}
