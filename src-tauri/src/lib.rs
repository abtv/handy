use open;
use std::process::Command;
use tauri::{AppHandle, Manager};

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    fs,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    urls: std::collections::HashMap<String, String>,
    programs: std::collections::HashMap<String, String>,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(load_config()));
}

fn load_config() -> Config {
    // TODO remove hardcoded string
    let yaml_str = fs::read_to_string("/Users/andrew/.config/handy/config.yaml")
        .expect("Failed to read config file");
    let config = serde_yaml::from_str(&yaml_str).expect("Failed to parse YAML");

    println!("{:#?}", config);

    return config;
}

#[tauri::command]
fn get_urls() -> Vec<String> {
    let config = CONFIG.lock().unwrap();
    let url_keys = config.urls.keys().cloned().collect();
    return url_keys;
}

#[tauri::command]
fn get_programs() -> Vec<String> {
    let config = CONFIG.lock().unwrap();
    let program_keys = config.programs.keys().cloned().collect();
    return program_keys;
}

#[tauri::command]
fn show_window(app: AppHandle) -> () {
    let window = app.get_webview_window("main").unwrap();
    window.show().unwrap();
    //window.set_always_on_top(true).unwrap();
    window.set_focus().unwrap();
}

#[tauri::command]
fn hide_window(app: AppHandle) -> () {
    let window = app.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

#[tauri::command]
fn open_url(name: &str) -> () {
    let config = CONFIG.lock().unwrap();
    match config.urls.get(name) {
        Some(&ref value) => {
            if let Err(e) = open::that(value) {
                eprintln!("Failed to open URL: {}", e);
            }
        }
        None => println!("Key '{}' not found", name),
    }
}

fn open_program(path: String) -> () {
    let output = Command::new("open")
        .arg("-a")
        .arg(path)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

#[tauri::command]
fn run_program(name: &str, app: AppHandle) -> () {
    let config = CONFIG.lock().unwrap();
    match config.programs.get(name) {
        Some(&ref value) => open_program(value.to_string()),
        None => println!("Key '{}' not found", name),
    }

    hide_window(app);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                // Don't show the app in the Dock
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);

                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_enter_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
                let app_handle = app.handle().clone();
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            let app_handle = app_handle.clone();
                            if shortcut == &ctrl_enter_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        show_window(app_handle);
                                    }
                                    ShortcutState::Released => {
                                        // No-op
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_enter_shortcut)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_window,
            hide_window,
            get_urls,
            get_programs,
            open_url,
            run_program
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
