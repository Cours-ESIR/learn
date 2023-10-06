// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlite::State;

#[tauri::command]
fn get() {
    let connection = sqlite::open("../private/db/test.db").unwrap();
    let mut query = connection.prepare("select * from users;").unwrap();

    while let Ok(State::Row) = query.next() {
        println!("name = {}", query.read::<String, _>("name").unwrap());
        println!("age = {}", query.read::<i64, _>("age").unwrap());
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
