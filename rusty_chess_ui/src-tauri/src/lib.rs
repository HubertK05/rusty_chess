use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};

use backend::{
    board_setup::models::{Board, FenNotation},
    config::AppSettings,
    move_generator::models::{MoveRestrictionData, Moves},
    move_register::models::ChessMove,
    opening_book::{move_parser::parse_move, OpeningBook},
};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use tauri::{
    async_runtime::{spawn, Mutex},
    path::BaseDirectory,
    AppHandle, Emitter, Listener, Manager,
};
use tokio::sync::Notify;

struct AppState {
    board: Mutex<Board>,
    repetition_map: Mutex<BTreeMap<u64, u8>>,
    opening_book: OnceLock<OpeningBook>,
    app_settings: OnceLock<Mutex<AppSettings>>,
    turn_counter: OnceLock<Arc<Mutex<u32>>>,
    toggled: Arc<Mutex<ToggleState>>,
    cvar: Notify,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
enum ToggleState {
    Idle,
    Running,
    Waiting,
}

enum GameOutcome {
    Ongoing,
    Done(String),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
enum CancelResult {
    Canceled,
    NotCanceled,
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

        app.emit("update-board", *board_guard)
            .map_err(|e| format!("Failed to send update-board event: {e:?}"))?;

        drop(board_guard);

        if let GameOutcome::Done(msg) = self.get_game_outcome().await {
            app.emit("end-game", msg)
                .map_err(|e| format!("Failed to send end-game event: {e:?}"))?;
        }

        *self.turn_counter.get().unwrap().lock().await += 1;
        *self.toggled.lock().await = ToggleState::Idle;
        self.cvar.notify_waiters();
        Ok(())
    }

    async fn get_game_outcome(&self) -> GameOutcome {
        let board_guard = self.board.lock().await;
        if board_guard.half_move_timer_50 > 100 {
            return GameOutcome::Done("Draw by the 50 move rule".into());
        } else if board_guard.mating_material.0 < 3 && board_guard.mating_material.1 < 3 {
            return GameOutcome::Done("Draw by insufficitent mating material".into());
        } else if Moves::get_all_moves(&board_guard, board_guard.turn)
            .0
            .is_empty()
        {
            if MoveRestrictionData::get(&board_guard, board_guard.turn)
                .check_squares
                .checks_amount
                != 0
            {
                return GameOutcome::Done(format!(
                    "{} wins by checkmate",
                    board_guard.turn.opp().to_string()
                ));
            } else {
                return GameOutcome::Done("Draw by stalemate".into());
            }
        } else if self
            .repetition_map
            .lock()
            .await
            .iter()
            .any(|(_pos, occurences)| *occurences >= 3)
        {
            return GameOutcome::Done("Draw by threefold repetition".into());
        } else {
            return GameOutcome::Ongoing;
        }
    }

    async fn restart(&self) {
        *self.board.lock().await = Board::new_game();
        *self.repetition_map.lock().await = BTreeMap::new();
        *self.toggled.lock().await = ToggleState::Idle;
        *self.turn_counter.get().unwrap().lock().await += 1;
        self.cvar.notify_waiters();
    }
}

#[tauri::command]
async fn autoplay_move(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<CancelResult, String> {
    let previous_toggle_state = { *state.toggled.lock().await };
    // println!("{previous_toggle_state:?}");
    *state.toggled.lock().await = ToggleState::Running;
    if previous_toggle_state != ToggleState::Idle {
        state.cvar.notify_waiters();
        return Ok(CancelResult::Canceled);
    }

    *state.toggled.lock().await = ToggleState::Running;

    let board_guard = state.board.lock().await;
    let fen = FenNotation::from(&*board_guard);

    let board = board_guard.clone();
    drop(board_guard);

    let starting_turn_count = { *state.turn_counter.get().unwrap().lock().await };

    let chosen_move =
        if let Some(move_vec) = state.opening_book.get().unwrap().0.get(&fen.to_draw_fen()) {
            let mut rng = thread_rng();
            let san = move_vec
                .choose_weighted(&mut rng, |(_, popularity)| *popularity)
                .unwrap();
            let res = parse_move(fen, san.0.clone()).expect("cannot parse move");

            Some(res)
        } else {
            let repetition_map = state.repetition_map.lock().await.clone();
            let app_settings = { state.app_settings.get().unwrap().lock().await.clone() };

            let thread = std::thread::spawn(move || {
                backend::chess_bot::choose_move(&board, repetition_map, app_settings)
            });
            thread.join().unwrap()
        };

    let Some(chosen_move) = chosen_move else {
        return Err("Failed to choose move".into());
    };

    if *state.toggled.lock().await == ToggleState::Waiting {
        println!("Waiting");
        state.cvar.notified().await;
        println!("Notified");
    }

    let is_redundant = { *state.turn_counter.get().unwrap().lock().await != starting_turn_count };

    if is_redundant {
        return Ok(CancelResult::Canceled);
    }

    state.play_move_loudly(app, chosen_move).await?;
    // println!("Executed successfully");
    Ok(CancelResult::NotCanceled)
}

#[tauri::command]
async fn get_legal_moves(state: tauri::State<'_, AppState>) -> Result<Moves, ()> {
    let board_guard = state.board.lock().await;
    Ok(Moves::get_all_moves(&board_guard, board_guard.turn))
}

#[tauri::command]
async fn play_move_manually(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    move_to_play: ChessMove,
) -> Result<(), String> {
    state.play_move_loudly(app, move_to_play).await
}

#[tauri::command]
async fn restart_game(state: tauri::State<'_, AppState>) -> Result<(), ()> {
    state.restart().await;
    Ok(())
}

#[tauri::command]
async fn update_settings(
    state: tauri::State<'_, AppState>,
    new_settings: AppSettings,
) -> Result<(), ()> {
    *state.app_settings.get().unwrap().lock().await = new_settings;
    Ok(())
}

#[tauri::command]
async fn get_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, ()> {
    return Ok(*state.app_settings.get().unwrap().lock().await);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            board: Mutex::new(Board::new_game()),
            repetition_map: Mutex::new(BTreeMap::new()),
            opening_book: OnceLock::new(),
            app_settings: OnceLock::new(),
            turn_counter: OnceLock::new(),
            toggled: Arc::new(Mutex::new(ToggleState::Idle)),
            cvar: Notify::new(),
        })
        .setup(|app| {
            let opening_book_path = app
                .path()
                .resolve("resources/opening_book.txt", BaseDirectory::Resource)
                .unwrap();

            let settings_path = app
                .path()
                .resolve("resources/settings.toml", BaseDirectory::Resource)
                .unwrap();

            app.state::<AppState>()
                .opening_book
                .set(OpeningBook::from_file(
                    opening_book_path.as_os_str().to_str().unwrap(),
                ))
                .unwrap();

            app.state::<AppState>()
                .app_settings
                .set(Mutex::new(
                    AppSettings::get_from_file(settings_path.as_os_str().to_str().unwrap())
                        .unwrap(),
                ))
                .unwrap();

            let counter = Arc::new(Mutex::new(0));
            app.state::<AppState>()
                .turn_counter
                .set(Arc::clone(&counter))
                .expect("Attempted to set the channel more than once");

            let toggled = app.state::<AppState>().toggled.clone();
            app.listen("cancel-move", move |_| {
                let toggled = toggled.clone();
                spawn(async move {
                    let mut guard = toggled.lock().await;
                    if *guard != ToggleState::Idle {
                        *guard = ToggleState::Waiting;
                    }
                });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play_move_manually,
            autoplay_move,
            get_legal_moves,
            restart_game,
            update_settings,
            get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
