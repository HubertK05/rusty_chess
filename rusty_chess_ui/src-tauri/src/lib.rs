use std::{collections::BTreeMap, sync::OnceLock};

use backend::{
    board_setup::models::{Board, FenNotation},
    config::AppSettings,
    move_generator::models::{MoveRestrictionData, Moves},
    move_register::models::ChessMove,
    opening_book::{move_parser::parse_move, OpeningBook},
};
use rand::{seq::SliceRandom, thread_rng};
use tauri::{
    async_runtime::{channel, Mutex, Receiver, Sender},
    AppHandle, Emitter, Listener, Manager,
};

struct AppState {
    board: Mutex<Board>,
    repetition_map: Mutex<BTreeMap<u64, u8>>,
    opening_book: OpeningBook,
    app_settings: AppSettings,
    cancel_channel: OnceLock<Mutex<tauri::async_runtime::Receiver<()>>>,
}

enum GameOutcome {
    Ongoing,
    Done(String),
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

        drop(board_guard);

        if let GameOutcome::Done(msg) = self.get_game_outcome().await {
            app.emit("end-game", msg)
                .map_err(|e| format!("Failed to send end-game event: {e:?}"))?;
        }

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
    }
}

#[tauri::command]
async fn autoplay_move(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<ChessMove, String> {
    println!("Autoplaying move");
    let board_guard = state.board.lock().await;
    let fen = FenNotation::from(&*board_guard);

    let board = board_guard.clone();
    drop(board_guard);

    let _ = { state.cancel_channel.get().unwrap().lock().await.try_recv() };

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
            &board,
            state.repetition_map.lock().await.clone(),
            state.app_settings,
        )
    };

    println!("Checking cancel");
    let is_canceled = {
        state
            .cancel_channel
            .get()
            .unwrap()
            .lock()
            .await
            .try_recv()
            .ok()
            .is_some()
    };

    if is_canceled {
        println!("Canceled");
        return Err("Autoplay move canceled".into());
    }

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            board: Mutex::new(Board::new_game()),
            repetition_map: Mutex::new(BTreeMap::new()),
            opening_book: OpeningBook::from_file("opening_book.txt"),
            app_settings: AppSettings::get_from_file("settings.toml").unwrap(),
            cancel_channel: OnceLock::new(),
        })
        .setup(|app| {
            let (sender, receiver): (Sender<()>, Receiver<()>) = channel(1);
            app.state::<AppState>()
                .cancel_channel
                .set(Mutex::new(receiver))
                .expect("Attempted to set the channel more than once");
            app.listen("cancel-move", move |_| {
                let _ = sender.try_send(());
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play_move_manually,
            autoplay_move,
            get_legal_moves,
            restart_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
