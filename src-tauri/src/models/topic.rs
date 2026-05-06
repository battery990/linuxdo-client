use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    #[serde(default)]
    pub avatar_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicSummary {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub fancy_title: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub posts_count: u32,
    #[serde(default)]
    pub views: u32,
    #[serde(default)]
    pub like_count: u32,
    #[serde(default)]
    pub last_posted_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub category_id: u64,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub last_poster: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicListResponse {
    pub topic_list: TopicList,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicList {
    pub topics: Vec<TopicSummary>,
    #[serde(default)]
    pub more_topics_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub post_number: u32,
    #[serde(default)]
    pub cooked: String,
    #[serde(default)]
    pub raw: String,
    pub created_at: String,
    #[serde(default)]
    pub like_count: u32,
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicDetail {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub fancy_title: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub posts_count: u32,
    #[serde(default)]
    pub views: u32,
    #[serde(default)]
    pub like_count: u32,
    pub created_at: String,
    #[serde(default)]
    pub category_id: u64,
    pub post_stream: PostStream,
    #[serde(default)]
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostStream {
    pub posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: u64,
    pub username: String,
    #[serde(default)]
    pub post_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub topics: Vec<TopicSummary>,
    pub posts: Vec<Post>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListResponse {
    pub category_list: CategoryList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    pub topic_id: u64,
    pub title: String,
    pub saved_at: String,
}
