use crate::cli::Reel;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const API_URL: &str = "https://www.omdbapi.com/?apikey=";
const DEFAULT_KEY: &str = "4387ea2a"; // Replace with your own key (they are free!)

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TitleData {
    pub title: Option<String>,
    pub director: Option<String>,
    pub year: Option<String>,
    pub runtime: Option<String>,
    pub genre: Option<String>,
    pub writer: Option<String>,
    pub released: Option<String>,
    pub actors: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub metascore: Option<String>,

    #[serde(rename = "imdbRating")]
    pub imdb_rating: Option<String>,

    #[serde(rename = "imdbID")]
    pub imdb_id: Option<String>,

    pub box_office: Option<String>,
    pub rated: Option<String>,
    pub awards: Option<String>,
    pub plot: Option<String>,
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

pub struct Column {
    pub header: &'static str,
    pub enabled: fn(&Reel) -> bool,
    pub value: fn(&TitleData) -> String,
}

fn opt(v: &Option<String>) -> String {
    v.clone().unwrap_or_else(|| "—".to_string())
}

pub fn get_columns() -> Vec<Column> {
    vec![
        Column {
            header: "Title",
            enabled: |_| true,
            value: |d| opt(&d.title),
        },
        Column {
            header: "Director",
            enabled: |_| true,
            value: |d| opt(&d.director),
        },
        Column {
            header: "Year",
            enabled: |_| true,
            value: |d| opt(&d.year),
        },
        Column {
            header: "Runtime",
            enabled: |_| true,
            value: |d| opt(&d.runtime),
        },
        Column {
            header: "Genre",
            enabled: |_| true,
            value: |d| opt(&d.genre),
        },
        Column {
            header: "Writer",
            enabled: |r| r.writer,
            value: |d| opt(&d.writer),
        },
        Column {
            header: "Released",
            enabled: |r| r.released,
            value: |d| opt(&d.released),
        },
        Column {
            header: "Actors",
            enabled: |r| r.actors,
            value: |d| opt(&d.actors),
        },
        Column {
            header: "Language",
            enabled: |r| r.language,
            value: |d| opt(&d.language),
        },
        Column {
            header: "Country",
            enabled: |r| r.country,
            value: |d| opt(&d.country),
        },
        Column {
            header: "Metascore",
            enabled: |r| r.metascore,
            value: |d| opt(&d.metascore),
        },
        Column {
            header: "IMDb Rating",
            enabled: |r| r.imdb,
            value: |d| opt(&d.imdb_rating),
        },
        Column {
            header: "Box Office",
            enabled: |r| r.box_office,
            value: |d| opt(&d.box_office),
        },
        Column {
            header: "MPA Rating",
            enabled: |r| r.rating,
            value: |d| opt(&d.rated),
        },
        Column {
            header: "Awards",
            enabled: |r| r.awards,
            value: |d| opt(&d.awards),
        },
        Column {
            header: "Plot",
            enabled: |r| r.plot,
            value: |d| opt(&d.plot),
        }
    ]
}
