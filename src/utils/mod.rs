use crate::store::inmemory::Store;
use chrono::Local;
use std::path::PathBuf;
use std::process::Command;

pub fn get_resource_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    path.push(file_name);
    path
}

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
        // Используем PowerShell для показа оповещения на Windows
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
        // Используем osascript для показа оповещения на macOS
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
    if cfg!(target_os = "windows") {
        // Windows 10+ уведомления через PowerShell
        let ps_script = format!(
            "[reflection.assembly]::loadwithpartialname('System.Windows.Forms'); \
             [reflection.assembly]::loadwithpartialname('System.Drawing'); \
             $notify = New-Object System.Windows.Forms.NotifyIcon; \
             $notify.Icon = [System.Drawing.SystemIcons]::Information; \
             $notify.BalloonTipTitle = '{}'; \
             $notify.BalloonTipText = '{}'; \
             $notify.Visible = $True; \
             $notify.ShowBalloonTip(5000);",
            title, message
        );

        Command::new("powershell")
            .arg("-Command")
            .arg(&ps_script)
            .output()
            .map_err(|e| format!("Не удалось выполнить PowerShell команду: {}", e))?;
    } else if cfg!(target_os = "macos") {
        // macOS уведомления через osascript
        let apple_script = format!(
            "display notification \"{}\" with title \"{}\"",
            message, title
        );

        Command::new("osascript")
            .arg("-e")
            .arg(&apple_script)
            .output()
            .map_err(|e| format!("Не удалось выполнить AppleScript команду: {}", e))?;
    } else {
        return Err("Платформа не поддерживается".to_string());
    }

    Ok(())
}

pub async fn check_scheduled_events(
    store: &Store,
    title: &str,
    interval_sec: u64,
) -> Result<(), String> {
    let time_now = get_now_as_string_as_time();
    if let Some(event) = store.get(&time_now) {
        show_system_notification(title, &event)?;
    }
    tokio::time::sleep(std::time::Duration::from_secs(interval_sec)).await;
    Ok(())
}
