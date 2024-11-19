use std::collections::BTreeMap;

use backend::{
    board_setup::models::{Board, FenNotation},
    config::AppSettings,
    move_generator::models::Moves,
    move_register::models::ChessMove,
    opening_book::{move_parser::parse_move, OpeningBook},
};
use rand::{seq::SliceRandom, thread_rng};
use tauri::{async_runtime::Mutex, AppHandle, Emitter};

struct AppState {
    board: Mutex<Board>,
    repetition_map: Mutex<BTreeMap<u64, u8>>,
    opening_book: OpeningBook,
    app_settings: AppSettings,
}

impl AppState {
    /// Plays a given move, while also sending an event to update the board in the frontend.
    async fn play_move_loudly(
        &self,
        app: AppHandle,
        move_to_play: ChessMove,
    ) -> Result<(), String> {
        let mut board_guard = self.board.lock().await;

        board_guard
            .register_move(move_to_play)
            .map_err(|e| format!("Error registering move: {e:?}"))?;

        let board_hash = board_guard.hash_board();

        self.repetition_map
            .lock()
            .await
            .entry(board_hash)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        println!("Attempting to emit event");
        app.emit("update-board", *board_guard)
            .map_err(|e| format!("Failed to send update-board event: {e:?}"))?;

        Ok(())
    }
}

#[tauri::command]
async fn autoplay_move(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<ChessMove, String> {
    let board_guard = state.board.lock().await;
    let fen = FenNotation::from(&*board_guard);
    let chosen_move = if let Some(move_vec) = state.opening_book.0.get(&fen.to_draw_fen()) {
        let mut rng = thread_rng();
        let san = move_vec
            .choose_weighted(&mut rng, |(_, popularity)| *popularity)
            .unwrap();
        let res = parse_move(fen, san.0.clone()).expect("cannot parse move");
        println!("played book move");

        Some(res)
    } else {
        backend::chess_bot::choose_move(
            &board_guard,
            state.repetition_map.lock().await.clone(),
            state.app_settings,
        )
    };

    drop(board_guard);

    if let Some(m) = chosen_move {
        state.play_move_loudly(app, m).await?;
    }

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
            repetition_map: Mutex::new(BTreeMap::new()),
            opening_book: OpeningBook::from_file("opening_book.txt"),
            app_settings: AppSettings::get_from_file("settings.toml").unwrap(),
        })
        .invoke_handler(tauri::generate_handler![autoplay_move, get_legal_moves])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
