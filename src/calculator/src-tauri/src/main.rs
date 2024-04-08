#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calculate(equation: String) -> String {
    //println!("esntaeit");
    equation.replace("π","3.14159265358979323846264338327950288").replace("e","2.71828182845904523536028747135266250")
    
    //Error".into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

