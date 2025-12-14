mod api;
mod cli;

use clap::Parser;
use comfy_table::{
    Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, modifiers::UTF8_SOLID_INNER_BORDERS,
    presets::UTF8_FULL,
};
use reqwest::Client;

use api::TitleData;
use api::fetch_title_data;
use cli::Reel;

struct Column {
    header: &'static str,
    enabled: fn(&Reel) -> bool,
    value: fn(&TitleData) -> String,
}

fn columns() -> Vec<Column> {
    vec![
        Column {
            header: "Title",
            enabled: |_| true,
            value: |d| d.title.clone(),
        },
        Column {
            header: "Director",
            enabled: |_| true,
            value: |d| d.director.clone(),
        },
        Column {
            header: "Year",
            enabled: |_| true,
            value: |d| d.year.clone(),
        },
        Column {
            header: "Runtime",
            enabled: |_| true,
            value: |d| d.runtime.clone(),
        },
        Column {
            header: "Genre",
            enabled: |_| true,
            value: |d| d.genre.clone(),
        },
        Column {
            header: "Writer",
            enabled: |r| r.writer,
            value: |d| d.writer.clone(),
        },
        Column {
            header: "Released",
            enabled: |r| r.released,
            value: |d| d.released.clone(),
        },
        Column {
            header: "Actors",
            enabled: |r| r.actors,
            value: |d| d.actors.clone(),
        },
        Column {
            header: "Language",
            enabled: |r| r.language,
            value: |d| d.language.clone(),
        },
        Column {
            header: "Country",
            enabled: |r| r.country,
            value: |d| d.country.clone(),
        },
        Column {
            header: "Metascore",
            enabled: |r| r.metascore,
            value: |d| d.metascore.clone(),
        },
        Column {
            header: "IMDb Rating",
            enabled: |r| r.imdb,
            value: |d| d.imdb_rating.clone(),
        },
        Column {
            header: "Box Office",
            enabled: |r| r.box_office,
            value: |d| d.box_office.clone(),
        },
        Column {
            header: "MPA Rating",
            enabled: |r| r.rating,
            value: |d| d.rated.clone(),
        },
    ]
}

#[tokio::main]
async fn main() {
    let args = Reel::parse();
    let client = Client::new();

    let queries: &[String] = &args.queries;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    let active_columns: Vec<Column> = columns()
        .into_iter()
        .filter(|c| (c.enabled)(&args))
        .collect();

    table.set_header(
        active_columns
            .iter()
            .map(|c| Cell::new(c.header))
            .collect::<Vec<_>>(),
    );

    for query in queries {
        let year = if queries.len() == 1 { args.year } else { None };

        match fetch_title_data(&client, query, year).await {
            Ok(data) => {
                table.add_row(
                    active_columns
                        .iter()
                        .map(|c| {
                            let mut cell = Cell::new((c.value)(&data));

                            if c.header == "IMDb Rating" {
                                let rating: f32 = data.imdb_rating.parse().unwrap_or(0.0);
                                let color = if rating >= 7.0 {
                                    Color::Green
                                } else if rating >= 4.0 {
                                    Color::Yellow
                                } else {
                                    Color::Red
                                };
                                cell = cell.fg(color);
                            }

                            if c.header == "Title" {
                                cell = cell.fg(Color::Cyan);
                            }

                            if c.header == "Metascore" {
                                let rating: u32 = data.metascore.parse().unwrap_or(0);
                                let color = if rating >= 70 {
                                    Color::Green
                                } else if rating >= 40 {
                                    Color::Yellow
                                } else {
                                    Color::Red
                                };
                                cell = cell.fg(color);
                            }

                            cell
                        })
                        .collect::<Vec<_>>(),
                );
            }
            Err(e) => eprintln!("Error fetching title: {}", e),
        }
    }

    if !table.is_empty() {
        println!("{table}");
    }
}
