use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const API_URL: &str = "https://www.omdbapi.com/?apikey=";
const DEFAULT_KEY: &str = "4387ea2a"; // Replace with your own key (they are free!)

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TitleData {
    pub title: String,
    pub director: String,
    pub year: String,
    pub runtime: String,
    pub genre: String,
    pub writer: String,
    pub released: String,
    pub actors: String,
    pub language: String,
    pub country: String,
    pub metascore: String,
    #[serde(rename = "imdbRating")]
    pub imdb_rating: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    pub box_office: String,
    pub rated: String,
}

#[derive(Debug, Error)]
pub enum TitleError {
    #[error("Request error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub async fn fetch_title_data(
    client: &Client,
    title: &str,
    year: Option<u16>,
) -> Result<TitleData, TitleError> {
    let encoded = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
    let mut url = format!("{}{}&t={}", API_URL, DEFAULT_KEY, encoded);

    if let Some(y) = year {
        use std::fmt::Write;
        write!(url, "&y={y}").unwrap();
    }

    let response = client.get(&url).send().await?.error_for_status()?;
    let data = response.json::<TitleData>().await?;

    Ok(data)
}
