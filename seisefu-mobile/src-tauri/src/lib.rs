use tauri::{State, Manager};
use std::sync::Mutex;
use std::fs;


use kyushu_core::{GameState, SerializableGameState};

pub struct AppState(Mutex<GameState>);

#[tauri::command]
fn get_game_state(state: State<AppState>) -> SerializableGameState {
    let game = state.0.lock().unwrap();
    game.to_serializable()
}

#[tauri::command]
fn click_hex(q: i32, r: i32, state: State<AppState>) -> SerializableGameState {
    let mut game = state.0.lock().unwrap();
    game.click_hex(q, r);
    game.to_serializable()
}

#[tauri::command]
fn handle_input(key: String, state: State<AppState>) -> SerializableGameState {
    let mut game = state.0.lock().unwrap();
    game.handle_input(&key);
    game.to_serializable()
}

#[tauri::command]
fn run_ai_turn(state: State<AppState>) -> SerializableGameState {
    let mut game = state.0.lock().unwrap();
    game.run_ai_turn();
    game.to_serializable()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let resource_path = app.path().resolve("resources/kyushu_data.json", tauri::path::BaseDirectory::Resource)?;
            let json_str = fs::read_to_string(resource_path).expect("failed to read kyushu_data.json");
            
            let mut game = GameState::new();
            game.load_json(&json_str).expect("failed to load game data");
            
            app.manage(AppState(Mutex::new(game)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_game_state, 
            click_hex, 
            handle_input, 
            run_ai_turn
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
