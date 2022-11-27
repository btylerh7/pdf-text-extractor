#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn extract_text(og_filepath: &str) -> String {
    let text = pdf_extract::extract_text(og_filepath)
        .expect("Not able to get text from doc");
    return text
}

#[tauri::command]
fn save_as_text(new_filepath: &str, text: &str) {
    use std::io::prelude:: *;
    let mut file = std::fs::File::create(String::from(new_filepath) + ".txt")
        .expect("Error encountered while creating file!");
    file.write_all(text.as_bytes())
        .expect("Unable to write file.");
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![extract_text, save_as_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
