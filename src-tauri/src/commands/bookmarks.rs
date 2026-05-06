use tauri::command;
use tauri::AppHandle;

#[command]
pub async fn add_bookmark(app: AppHandle, topic_id: u64, title: String) -> Result<(), String> {
    let db = get_db(&app).await?;
    db.execute(
        "INSERT OR IGNORE INTO bookmarks (topic_id, title) VALUES (?1, ?2)",
        vec![serde_json::to_value(topic_id).unwrap(), serde_json::to_value(&title).unwrap()],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn remove_bookmark(app: AppHandle, topic_id: u64) -> Result<(), String> {
    let db = get_db(&app).await?;
    db.execute(
        "DELETE FROM bookmarks WHERE topic_id = ?1",
        vec![serde_json::to_value(topic_id).unwrap()],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn is_bookmarked(app: AppHandle, topic_id: u64) -> Result<bool, String> {
    let db = get_db(&app).await?;
    let result: Vec<serde_json::Value> = db
        .select(
            "SELECT COUNT(*) as count FROM bookmarks WHERE topic_id = ?1",
            vec![serde_json::to_value(topic_id).unwrap()],
        )
        .await
        .map_err(|e| e.to_string())?;
    let count = result
        .first()
        .and_then(|r| r.get("count"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    Ok(count > 0)
}

#[command]
pub async fn get_bookmarks(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let db = get_db(&app).await?;
    let result = db
        .select(
            "SELECT id, topic_id, title, saved_at FROM bookmarks ORDER BY saved_at DESC",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

#[command]
pub async fn add_search_history(app: AppHandle, query: String) -> Result<(), String> {
    let db = get_db(&app).await?;
    db.execute(
        "INSERT INTO search_history (query) VALUES (?1)",
        vec![serde_json::to_value(&query).unwrap()],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn get_search_history(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let db = get_db(&app).await?;
    let result = db
        .select(
            "SELECT id, query, searched_at FROM search_history ORDER BY searched_at DESC LIMIT 20",
            vec![],
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

async fn get_db(app: &AppHandle) -> Result<tauri_plugin_sql::Database, String> {
    use tauri_plugin_sql::TauriSqlExt;
    app.sql()
        .get("sqlite:linuxdo.db")
        .await
        .map_err(|e| e.to_string())
}
