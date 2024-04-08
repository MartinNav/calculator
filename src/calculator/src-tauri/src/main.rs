#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use math_lib;

#[tauri::command]
fn calculate(equation: String) -> Result<String, String> {
    let equation = equation.replace("π","3.14159265358979323846264338327950288").replace("e","2.71828182845904523536028747135266250");
    math_lib::parse(equation.as_str())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
