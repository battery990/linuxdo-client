use tauri::command;

use crate::api::discourse::DiscourseClient;
use crate::models::topic::{TopicDetail, TopicListResponse};

#[command]
pub async fn get_topics(page: Option<u32>, category: Option<String>) -> Result<TopicListResponse, String> {
    let client = DiscourseClient::new();
    client
        .get_topics(page, category.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_topic_detail(topic_id: u64) -> Result<TopicDetail, String> {
    let client = DiscourseClient::new();
    client
        .get_topic_detail(topic_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn search_topics(query: String) -> Result<crate::api::discourse::SearchResponse, String> {
    let client = DiscourseClient::new();
    client.search(&query).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_categories() -> Result<crate::models::topic::CategoryListResponse, String> {
    let client = DiscourseClient::new();
    client.get_categories().await.map_err(|e| e.to_string())
}
