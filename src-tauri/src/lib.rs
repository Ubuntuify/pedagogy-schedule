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

    let path_resolver = app.path();
    let mut path = path_resolver.app_config_dir()?;

    path.push("settings");

    let mut path = path_resolver.app_data_dir()?;

    path.push("data");

    let db = tauri::async_runtime::block_on(tauri::async_runtime::spawn(async {
        let db: surrealdb::Surreal<surrealdb::engine::local::Db> = surrealdb::Surreal::init();

        #[cfg(!debug_assertions)]
        db.connect::<surrealdb::engine::local::RocksDb>(path)
            .await
            .map_err(|error| error.to_string())?;
        #[cfg(debug_assertions)]
        db.connect::<surrealdb::engine::local::Mem>("memory")
            .await
            .map_err(|error| error.to_string())?; // use in-memory database during development

        db.use_ns("schedule")
            .use_db("schedule")
            .await
            .map_err(|error| error.to_string());

        Ok(db)
    }));

    app.manage(db);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::class::list_classes,
            commands::class::upsert_class,
            commands::class::create_class,
        ])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
