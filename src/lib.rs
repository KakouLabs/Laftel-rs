#[cfg(feature = "async")]
pub mod client;
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod error;
pub mod models;

#[cfg(feature = "async")]
pub use client::LaftelClient;

pub use error::{LaftelError, Result};

pub use models::{
    AnimeInfo,
    SearchResult,
    SeriesSearchResult,
    SearchEpisode,
    SearchResponse,
    SeriesResponse,
    EpisodeResponse,
};
