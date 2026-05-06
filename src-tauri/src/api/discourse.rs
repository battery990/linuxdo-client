use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::models::topic::{
    CategoryListResponse, TopicDetail, TopicListResponse,
};

const BASE_URL: &str = "https://linux.do";

pub struct DiscourseClient {
    client: Client,
    base_url: String,
}

impl DiscourseClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("LinuxDoClient/0.1")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn get_topics(&self, page: Option<u32>, category: Option<&str>) -> Result<TopicListResponse> {
        let url = match category {
            Some(slug) => format!("{}/c/{}.json", self.base_url, slug),
            None => format!("{}/latest.json", self.base_url),
        };

        let mut req = self.client.get(&url);
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }

        let resp = req.send().await?.error_for_status()?;
        let data: TopicListResponse = resp.json().await?;
        Ok(data)
    }

    pub async fn get_topic_detail(&self, topic_id: u64) -> Result<TopicDetail> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let data: TopicDetail = resp.json().await?;
        Ok(data)
    }

    pub async fn get_topic_posts(&self, topic_id: u64, post_number: u32) -> Result<TopicDetail> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        let resp = self
            .client
            .get(&url)
            .query(&[("post_number", post_number.to_string())])
            .send()
            .await?
            .error_for_status()?;
        let data: TopicDetail = resp.json().await?;
        Ok(data)
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        let url = format!("{}/search.json", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await?
            .error_for_status()?;
        let data: SearchResponse = resp.json().await?;
        Ok(data)
    }

    pub async fn get_categories(&self) -> Result<CategoryListResponse> {
        let url = format!("{}/categories.json", self.base_url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let data: CategoryListResponse = resp.json().await?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub topics: Option<Vec<crate::models::topic::TopicSummary>>,
    pub posts: Option<Vec<crate::models::topic::Post>>,
    pub users: Option<Vec<crate::models::topic::User>>,
}
