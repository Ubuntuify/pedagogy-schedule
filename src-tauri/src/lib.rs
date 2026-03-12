use tauri::{App, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod database {
    pub mod structs;
}

pub mod commands {
    pub mod class;
    pub mod schedule;
}

mod utils;

fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        app.get_webview_window("main")
            .map(|window| window.open_devtools())
            .or_else(|| {
                eprintln!("dev: Failed to open devtools.");
                None
            });
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
