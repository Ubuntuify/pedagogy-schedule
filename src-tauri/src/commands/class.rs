use core::error;

use surrealdb::Surreal;

use crate::database::{self, structs::Class};

#[tauri::command]
pub async fn list_classes(
    db: State<'_, tokio::sync::Mutex<Surreal<Db>>>,
) -> Result<Vec<Class>, String> {
    let db: Surreal<Any> = db.lock().await;
    db.select("class").await.map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn upsert_class(
    db: State<'_, tokio::sync::Mutex<Surreal<Db>>>,
    class: Class,
) -> Result<(), String> {
    let db: Surreal<Any> = db.lock().await;
    db.upsert("class")
        .content(class)
        .await
        .map_err(|error| error.to_string())?;
}

#[tauri::command]
pub async fn create_class(
    db: State<'_, tokio::sync::Mutex<Surreal<Db>>>,
    class: Class,
) -> Result<(), String> {
    let db: Surreal<Any> = db.lock().await;
    db.create("class")
        .content(class)
        .await
        .map_err(|error| error.to_string())?;
}
