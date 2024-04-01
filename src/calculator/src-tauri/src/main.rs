#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn call_parser(input: String) -> String {
    math_lib::parse(&input)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, call_parser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

