// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

#[tauri::command]
fn get_questionnary_all(db: tauri::State<Database>) -> Vec<i32> {
	let connection = db.connection.lock().unwrap();

	let mut results: Vec<i32> = Vec::<i32>::with_capacity(db.count("questionnary"));
	let mut query = connection.prepare("SELECT * FROM questionnary q").unwrap();

	while let Ok(sqlite::State::Row) = query.next() {
		results.push(0);
	}

	results
}

struct Database {
	connection: std::sync::Mutex<sqlite::Connection>,
}

impl Database {

	fn count(&self, query: &str) -> usize {
		self.connection.lock().unwrap().prepare(format!("SELECT COUNT(*) FROM {}", query)).unwrap().read::<i64, usize>(0).unwrap().try_into().unwrap()
	}

}

fn main() {
	tauri::Builder::default()
		.manage(
			Database { connection: std::sync::Mutex::new(sqlite::open("../private/db/questionnaries.db").unwrap()) }
		)
		.invoke_handler(tauri::generate_handler![get_questionnary_all])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
