mod api;
mod commands;
mod db;
mod models;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::topics::get_topics,
            commands::topics::get_topic_detail,
            commands::topics::search_topics,
            commands::topics::get_categories,
            commands::bookmarks::add_bookmark,
            commands::bookmarks::remove_bookmark,
            commands::bookmarks::is_bookmarked,
            commands::bookmarks::get_bookmarks,
            commands::bookmarks::add_search_history,
            commands::bookmarks::get_search_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
