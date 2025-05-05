use std::path::PathBuf;

fn get_resource_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    path.push(file_name);
    path
}
