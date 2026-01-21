use crate::error::Result;
use crate::models::{AnimeInfo, EpisodeResponse, SearchEpisode, SearchResponse, SearchResult, SeriesResponse, SeriesSearchResult};
use reqwest::{header, blocking::Client};

const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1_4; like Mac OS X) AppleWebKit/600.11 (KHTML, like Gecko) Chrome/54.0.1486.383 Mobile Safari/600.8";

#[derive(Debug, Clone)]
pub struct LaftelBlockingClient {
    http_client: Client,
}


impl LaftelBlockingClient {
    pub fn new() -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert("laftel", header::HeaderValue::from_static("TeJava"));

        let http_client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Self { http_client })
    }

    pub fn get_anime_info(&self, id: u64) -> Result<AnimeInfo> {
        let url = format!("https://laftel.net/api/items/v2/{}/", id);
        
        let response = self.http_client
            .get(&url)
            .send()?
            .error_for_status()?;

        Ok(response.json()?)
    }

    pub fn search_anime(&self, query: &str) -> Result<Vec<SearchResult>> {
        let url = "https://laftel.net/api/search/v3/keyword/";
        
        let response = self.http_client
            .get(url)
            .query(&[("keyword", query)])
            .send()?
            .error_for_status()?;

        let search_response: SearchResponse = response.json()?;
        Ok(search_response.results)
    }

    pub fn search_series(&self, id: u64) -> Result<Vec<SeriesSearchResult>> {
        let url = format!("https://laftel.net/api/items/v2/series/{}/", id);
        
        let response = self.http_client
            .get(&url)
            .query(&[("limit", "50"), ("offset", "0")])
            .send()?
            .error_for_status()?;

        let series_response: SeriesResponse = response.json()?;
        Ok(series_response.results)
    }

    pub fn search_episodes(&self, id: u64) -> Result<Vec<SearchEpisode>> {
        let url = "https://laftel.net/api/episodes/v2/list/";
        
        let id_str = id.to_string();
        let response = self.http_client
            .get(url)
            .query(&[
                ("item_id", id_str.as_str()),
                ("sort", "oldest"),
                ("limit", "50"),
                ("show_playback_offset", "true"),
                ("offset", "0"),
            ])
            .send()?
            .error_for_status()?;

        let episode_response: EpisodeResponse = response.json()?;
        Ok(episode_response.results)
    }

    #[inline]
    pub fn get_info_from_search(&self, result: &SearchResult) -> Result<AnimeInfo> {
        self.get_anime_info(result.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let _client = LaftelBlockingClient::new().unwrap();
    }
}
