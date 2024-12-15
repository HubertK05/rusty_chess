// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(unix)]
    {
        let env = include_str!("../.env");
        dotenvy::from_read(env.as_bytes()).unwrap();
    }

    rusty_chess_ui_lib::run()
}
