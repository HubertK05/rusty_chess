use std::collections::BTreeMap;

use backend::{
    board_setup::models::Board, config::AppSettings, move_generator::models::Moves,
    move_register::models::ChessMove, opening_book::OpeningBook,
};
use tauri::async_runtime::Mutex;

struct AppState {
    board: Mutex<Board>,
    repetition_map: BTreeMap<u64, u8>,
    opening_book: OpeningBook,
    app_settings: AppSettings,
}

#[tauri::command]
async fn autoplay_move(state: tauri::State<'_, AppState>) -> Result<ChessMove, String> {
    let chosen_move = {
        let board_guard = state.board.lock().await;
        backend::chess_bot::choose_move(
            &board_guard,
            state.repetition_map.clone(),
            &state.opening_book,
            state.app_settings,
        )
    };

    return chosen_move.ok_or("Failed to choose move".into());
}

#[tauri::command]
async fn get_legal_moves(state: tauri::State<'_, AppState>) -> Result<Moves, ()> {
    let board_guard = state.board.lock().await;
    Ok(Moves::get_all_moves(&board_guard, board_guard.turn))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            board: Mutex::new(Board::new_game()),
            repetition_map: BTreeMap::new(),
            opening_book: OpeningBook::from_file("opening_book.txt"),
            app_settings: AppSettings::get_from_file("settings.toml").unwrap(),
        })
        .invoke_handler(tauri::generate_handler![autoplay_move, get_legal_moves])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
