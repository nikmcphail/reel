mod api;
mod cli;

use clap::Parser;
use comfy_table::{
    Cell, Color, Table,
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
};
use reqwest::Client;

use api::{Column, fetch_title_data, get_columns};
use cli::Reel;

#[tokio::main]
async fn main() {
    let args = Reel::parse();
    let client = Client::new();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    let active_columns: Vec<Column> = get_columns()
        .into_iter()
        .filter(|c| (c.enabled)(&args))
        .collect();

    table.set_header(active_columns.iter().map(|c| Cell::new(c.header)));

    for query in &args.queries {
        let year = if args.queries.len() == 1 {
            args.year
        } else {
            None
        };

        match fetch_title_data(&client, query, year).await {
            Ok(data) => {
                table.add_row(active_columns.iter().map(|c| {
                    let mut cell = Cell::new((c.value)(&data));

                    if c.header == "Title" {
                        cell = cell.fg(Color::Cyan);
                    }

                    if c.header == "IMDb Rating"
                        && let Some(r) = &data.imdb_rating
                    {
                        let rating: f32 = r.parse().unwrap_or(0.0);
                        let color = if rating >= 7.0 {
                            Color::Green
                        } else if rating >= 4.0 {
                            Color::Yellow
                        } else {
                            Color::Red
                        };
                        cell = cell.fg(color);
                    }

                    if c.header == "Metascore"
                        && let Some(m) = &data.metascore
                    {
                        let rating: u32 = m.parse().unwrap_or(0);
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
                }));
            }
            Err(e) => eprintln!("Error fetching title: {}", e),
        }
    }

    if !table.is_empty() {
        println!("{table}");
    }
}
