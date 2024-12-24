// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Disables the default renderer, which doesn't work with NVIDIA GPU on Linux
    // https://github.com/tauri-apps/tauri/issues/9304
    #[cfg(unix)]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    rusty_chess_ui_lib::run()
}
