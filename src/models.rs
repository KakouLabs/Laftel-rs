use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeInfo {
    pub id: u64,
    pub name: String,
    #[serde(rename = "img")]
    pub image: String,
    pub content: String,
    #[serde(rename = "is_ending")]
    pub ended: bool,
    #[serde(default)]
    pub awards: Vec<String>,
    #[serde(default)]
    pub content_rating: Option<String>,
    #[serde(rename = "is_adult")]
    pub adultonly: bool,
    #[serde(rename = "is_viewing")]
    pub viewable: bool,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, rename = "air_year_quarter")]
    pub air_year_quarter: Option<String>,
    #[serde(default, rename = "distributed_air_time")]
    pub air_day: Option<String>,
    #[serde(default)]
    pub avg_rating: f64,
    #[serde(default)]
    pub series_id: Option<u64>,
    #[serde(default)]
    pub production: Option<String>,
}

impl AnimeInfo {
    pub fn url(&self) -> String {
        format!("https://laftel.net/item/{}", self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    pub name: String,
    #[serde(rename = "img")]
    pub image: String,
    #[serde(rename = "is_adult")]
    pub adultonly: bool,
    #[serde(default)]
    pub genres: Vec<String>,
}

impl SearchResult {
    pub fn url(&self) -> String {
        format!("https://laftel.net/item/{}", self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesSearchResult {
    pub id: u64,
    pub name: String,
}

impl SeriesSearchResult {
    pub fn url(&self) -> String {
        format!("https://laftel.net/item/{}", self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEpisode {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub episode_num: String,
    pub episode_order: u64,
    pub thumbnail_path: String,
    #[serde(default)]
    pub running_time: Option<String>,
    #[serde(rename = "is_viewing")]
    pub is_available: bool,
}

impl SearchEpisode {
    pub fn asset_id(&self) -> Option<String> {
        self.thumbnail_path
            .split('/')
            .nth(4)
            .and_then(|_start| {
                 let parts = self.thumbnail_path.split('/').skip(4).take(3);
                 if parts.clone().count() == 3 {
                     Some(parts.collect::<Vec<&str>>().join("/"))
                 } else {
                     None
                 }
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesResponse {
    pub results: Vec<SeriesSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeResponse {
    pub results: Vec<SearchEpisode>,
}
