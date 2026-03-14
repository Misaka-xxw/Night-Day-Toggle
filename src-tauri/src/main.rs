#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 获取当前系统主题
#[tauri::command]
fn get_system_theme() -> String {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(personalization) =
            hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
        {
            if let Ok(value) = personalization.get_value::<u32, _>("AppsUseLightTheme") {
                return if value == 1 {
                    "light".to_string()
                } else {
                    "dark".to_string()
                };
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        match Command::new("defaults")
            .args(&["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            Ok(output) => {
                let theme = String::from_utf8_lossy(&output.stdout);
                if theme.trim() == "Dark" {
                    return "dark".to_string();
                }
            }
            Err(_) => {}
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        match Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
        {
            Ok(output) => {
                let theme = String::from_utf8_lossy(&output.stdout);
                if theme.to_lowercase().contains("dark") {
                    return "dark".to_string();
                }
            }
            Err(_) => {}
        }
    }

    "light".to_string()
}

// 切换系统主题
#[tauri::command]
fn toggle_system_theme() -> Result<String, String> {
    let current = get_system_theme();
    let target_dark = current == "light"; // 想变成 dark 就 true
    let value: u32 = if target_dark { 0 } else { 1 };

    #[cfg(target_os = "windows")]
    {
        use std::ffi::CString;
        use std::process::Command;
        use winapi::um::winuser::{PostMessageW, HWND_BROADCAST, WM_SETTINGCHANGE};
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";

        let mut key = match hkcu.open_subkey_with_flags(path, KEY_SET_VALUE) {
            Ok(k) => k,
            Err(e) => return Err(format!("无法打开 Personalize 键: {}", e)),
        };

        let value: u32 = if target_dark { 0 } else { 1 };

        if let Err(e) = key.set_value("AppsUseLightTheme", &value) {
            return Err(format!("写入 AppsUseLightTheme 失败: {}", e));
        }
        if let Err(e) = key.set_value("SystemUsesLightTheme", &value) {
            return Err(format!("写入 SystemUsesLightTheme 失败: {}", e));
        }

        let immersive = match CString::new("ImmersiveColorSet") {
            Ok(c) => c,
            Err(e) => return Err(format!("CString 失败: {}", e)),
        };

        unsafe {
            PostMessageW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0 as _,
                immersive.as_ptr() as isize,
            );
            PostMessageW(HWND_BROADCAST, WM_SETTINGCHANGE, 0 as _, 0 as isize);
        }

        std::thread::sleep(std::time::Duration::from_millis(200));
        let _ = Command::new("taskkill")
            .args(["/f", "/im", "explorer.exe"])
            .output();
        std::thread::sleep(std::time::Duration::from_millis(500));
        let _ = Command::new("explorer.exe").spawn();
        let after = get_system_theme();
        println!("切换后验证: {}", after);
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let script = if target_dark {
            "tell application \"System Events\" to tell appearance preferences to set dark mode to true"
        } else {
            "tell application \"System Events\" to tell appearance preferences to set dark mode to false"
        };

        match Command::new("osascript").args(&["-e", script]).output() {
            Ok(_) => println!(
                "macOS主题已切换为: {}",
                if target_dark { "dark" } else { "light" }
            ),
            Err(e) => return Err(format!("macOS主题切换失败: {}", e)),
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let theme = if target_dark {
            "'Adwaita-dark'"
        } else {
            "'Adwaita'"
        };

        match Command::new("gsettings")
            .args(&["set", "org.gnome.desktop.interface", "gtk-theme", theme])
            .output()
        {
            Ok(_) => println!(
                "Linux主题已切换为: {}",
                if target_dark { "dark" } else { "light" }
            ),
            Err(e) => return Err(format!("Linux主题切换失败: {}", e)),
        }
    }

    Ok(if target_dark { "dark" } else { "light" }.to_string())
}

// 窗口拖动命令 - 简化的正确版本
#[tauri::command]
fn start_window_dragging(window: tauri::Window) -> Result<(), String> {
    // 直接调用 window.start_dragging()，这个方法在所有平台都可用
    println!("调用窗口拖动命令");
    window
        .start_dragging()
        .map_err(|e| format!("窗口拖动失败: {}", e))?;
    println!("窗口拖动命令执行成功");
    Ok(())
}

// 获取窗口位置
#[tauri::command]
async fn get_window_position(window: tauri::Window) -> Result<(i32, i32), String> {
    match window.outer_position() {
        Ok(pos) => Ok((pos.x, pos.y)),
        Err(e) => Err(format!("获取窗口位置失败: {}", e)),
    }
}

// 设置窗口位置
#[tauri::command]
async fn set_window_position(window: tauri::Window, x: i32, y: i32) -> Result<(), String> {
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }))
        .map_err(|e| format!("设置窗口位置失败: {}", e))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_system_theme,
            toggle_system_theme,
            start_window_dragging,
            get_window_position,  // 添加获取窗口位置命令
            set_window_position   // 添加设置窗口位置命令
        ])
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时出错");
}