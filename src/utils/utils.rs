use crate::store::inmemory::Store;
use chrono::Local;
use notify_rust::{Notification, Timeout};
use std::path::PathBuf;
use std::process::Command;

pub fn get_resource_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    path.push(file_name);
    path
}

#[allow(dead_code)]
pub fn get_now_as_string_as_date() -> String {
    let time = Local::now();
    time.format("%Y-%m-%d %H:%M").to_string()
}

pub fn get_now_as_string_as_time() -> String {
    let time = Local::now();
    time.format("%H:%M").to_string()
}

pub fn show_system_alert(message: &str) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        let ps_script = format!(
            "[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); \
             [System.Windows.Forms.MessageBox]::Show('{}')",
            message
        );

        Command::new("powershell")
            .arg("-Command")
            .arg(&ps_script)
            .output()
            .map_err(|e| format!("Failed to execute PowerShell command: {}", e))?;
    } else if cfg!(target_os = "macos") {
        let apple_script = format!(
            "display dialog \"{}\" with title \"Уведомление\" buttons {{\"OK\"}} default button \"OK\"",
            message
        );

        Command::new("osascript")
            .arg("-e")
            .arg(&apple_script)
            .output()
            .map_err(|e| format!("Failed to execute AppleScript command: {}", e))?;
    } else {
        return Err("Unsupported platform".to_string());
    }

    Ok(())
}

pub fn show_system_notification(title: &str, message: &str) -> Result<(), String> {
    Notification::new()
        .summary(title)
        .body(message)
        .timeout(Timeout::Milliseconds(5000))
        .show()
        .unwrap();
    Ok(())
}

pub async fn check_scheduled_events(store: &Store, interval_sec: u64) -> Result<(), String> {
    let time_now = get_now_as_string_as_time();
    if let Some(event) = store.get(&time_now) {
        match event.message_type.as_str() {
            "info" => show_system_notification(&event.title, &event.text)?,
            "warn" => show_system_alert(&event.text)?,
            _ => return Err("Invalid message type".to_string()),
        }
    }
    tokio::time::sleep(std::time::Duration::from_secs(interval_sec)).await;
    Ok(())
}
