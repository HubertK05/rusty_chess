use std::{collections::BTreeMap, sync::Arc};

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
use tokio::sync::{
    broadcast::{Receiver, Sender},
    Notify,
};

struct AppState {
    board: Mutex<Board>,
    repetition_map: Mutex<BTreeMap<u64, u8>>,
    opening_book: OpeningBook,
    app_settings: Mutex<AppSettings>,
    turn_counter: Arc<Mutex<u32>>,
    toggled: Arc<Mutex<ToggleState>>,
    cvar: Notify,
    chooser: Mutex<MoveChooser>,
    cancel_channel: Sender<()>,
}

struct MoveChooser {
    cancel_channel: Receiver<()>,
}

impl MoveChooser {
    async fn choose_move(&self, state: &AppState) -> Option<ChessMove> {
        let board_guard = state.board.lock().await;
        let fen = FenNotation::from(&*board_guard);

        let board = board_guard.clone();
        drop(board_guard);

        if let Some(move_vec) = state.opening_book.0.get(&fen.to_draw_fen()) {
            let mut rng = thread_rng();
            let san = move_vec
                .choose_weighted(&mut rng, |(_, popularity)| *popularity)
                .unwrap();
            let res = parse_move(fen, san.0.clone()).expect("cannot parse move");

            Some(res)
        } else {
            let repetition_map = state.repetition_map.lock().await.clone();
            let app_settings = { state.app_settings.lock().await.clone() };
            let mut cloned_channel = self.cancel_channel.resubscribe();

            let thread = std::thread::spawn(move || {
                backend::chess_bot::choose_move_cancelable(
                    &board,
                    repetition_map,
                    app_settings,
                    &mut cloned_channel,
                )
            });
            thread.join().unwrap()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
enum ToggleState {
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
    async fn get_toggle_state(&self) -> ToggleState {
        *self.toggled.lock().await
    }

    async fn set_toggle_state(&self, value: ToggleState) {
        *self.toggled.lock().await = value
    }

    async fn get_turn_count(&self) -> u32 {
        *self.turn_counter.lock().await
    }

    async fn increment_turn_count(&self) {
        *self.turn_counter.lock().await += 1
    }

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

        self.increment_turn_count().await;
        let _ = self.cancel_channel.send(());
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
        self.increment_turn_count().await;
        let _ = self.cancel_channel.send(());
        self.cvar.notify_waiters();
    }
}

#[tauri::command]
async fn autoplay_move(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<CancelResult, String> {
    state.set_toggle_state(ToggleState::Running).await;
    let Some(chooser_guard) = state.chooser.try_lock().ok() else {
        state.cvar.notify_waiters();
        return Ok(CancelResult::Canceled);
    };

    let starting_turn_count = state.get_turn_count().await;

    let chosen_move = chooser_guard.choose_move(&state).await;

    let Some(chosen_move) = chosen_move else {
        return Ok(CancelResult::Canceled);
    };

    if state.get_toggle_state().await == ToggleState::Waiting {
        state.cvar.notified().await;
    }

    if state.get_turn_count().await != starting_turn_count {
        return Ok(CancelResult::Canceled);
    }

    state.play_move_loudly(app, chosen_move).await?;
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
    *state.app_settings.lock().await = new_settings;
    Ok(())
}

#[tauri::command]
async fn get_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, ()> {
    return Ok(*state.app_settings.lock().await);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let opening_book_path = app
                .path()
                .resolve("resources/opening_book.txt", BaseDirectory::Resource)
                .unwrap();
            let opening_book =
                OpeningBook::from_file(opening_book_path.as_os_str().to_str().unwrap());

            let settings_path = app
                .path()
                .resolve("resources/settings.toml", BaseDirectory::Resource)
                .unwrap();
            let settings =
                AppSettings::get_from_file(settings_path.as_os_str().to_str().unwrap()).unwrap();

            let toggled = Arc::new(Mutex::new(ToggleState::Running));
            let (sender, receiver) = tokio::sync::broadcast::channel(1);
            app.manage(AppState {
                board: Mutex::new(Board::new_game()),
                repetition_map: Mutex::new(BTreeMap::new()),
                opening_book,
                app_settings: Mutex::new(settings),
                turn_counter: Arc::new(Mutex::new(0)),
                toggled: toggled.clone(),
                cvar: Notify::new(),
                chooser: Mutex::new(MoveChooser {
                    cancel_channel: receiver,
                }),
                cancel_channel: sender,
            });

            app.listen("cancel-move", move |_| {
                let toggled = toggled.clone();
                spawn(async move {
                    *toggled.lock().await = ToggleState::Waiting;
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
