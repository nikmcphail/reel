/// Base URL for OMDb API
const API_URL: &str = "http://www.omdbapi.com/?apikey=";
/// Default key for OMDb API.
const DEFAULT_KEY: &str = "5e540903";

use clap::Parser;
/// CLI arguments for querying movie and series information.
#[derive(Parser)]
#[command(about)]
struct Reel {
    /// Film or series title.
    #[arg(help = "Film or series title.")]
    query: String,

    /// Specify release year.
    #[arg(short = 'y', long = "year", help = "Specify release year.")]
    year: Option<u16>,

    /// Show the writer(s).
    #[arg(short = 'w', long = "writer", help = "Show the writer(s).")]
    writer: bool,
    /// Show the release date.
    #[arg(short = 'r', long = "released", help = "Show the release date.")]
    released: bool,
    /// Show the main cast.
    #[arg(short = 'a', long = "actors", help = "Show the main cast.")]
    actors: bool,
    /// Show the plot summary.
    #[arg(short = 'p', long = "plot", help = "Show the plot summary.")]
    plot: bool,
    /// Show the language(s).
    #[arg(short = 'l', long = "language", help = "Show the language(s).")]
    language: bool,
    /// Show the country(ies) of production.
    #[arg(
        short = 'c',
        long = "country",
        help = "Show the country(ies) of production."
    )]
    country: bool,
    /// Show the Metacritic score.
    #[arg(short = 'm', long = "metascore", help = "Show the Metacritic score.")]
    metascore: bool,
    /// Show the IMDb rating.
    #[arg(short = 'i', long = "imdb-rating", help = "Show the IMDb rating.")]
    imdb_rating: bool,
    /// Show the Rotten Tomatoes score.
    #[arg(
        short = 't',
        long = "tomato-meter",
        help = "Show the Rotten Tomatoes score."
    )]
    tomato_meter: bool,
    /// Show the box office earnings.
    #[arg(
        short = 'b',
        long = "box-office",
        help = "Show the box office earnings."
    )]
    box_office: bool,
    /// Show the MPA rating.
    #[arg(short = 'R', long = "rated", help = "Show the MPA rating.")]
    rated: bool,
    /// Show award wins and nominations.
    #[arg(
        short = 'A',
        long = "awards",
        help = "Show award wins and nominations."
    )]
    awards: bool,
    /// Show poster link.
    #[arg(short = 'P', long = "poster", help = "Show poster link.")]
    poster: bool,
}

/// Build a vector of properties to diplay based on user-selected flags.
///
/// Always includes `Title`, `Director`, `Year`, `Runtime`, and `Genre`.
///
/// # Arguments
///
/// * `args` - Reference to the `Reel` struct containing user input.
///
/// # Returns
///
/// A vector of static string slices representing the fields to display.
fn build_props(args: &Reel) -> Vec<&'static str> {
    let mut props = vec!["Title", "Director", "Year", "Runtime", "Genre"];

    if args.writer {
        props.push("Writer");
    }
    if args.released {
        props.push("Released");
    }
    if args.actors {
        props.push("Actors");
    }
    if args.plot {
        props.push("Plot");
    }
    if args.language {
        props.push("Language");
    }
    if args.country {
        props.push("Country");
    }
    if args.metascore {
        props.push("Metascore");
    }
    if args.imdb_rating {
        props.push("imdbRating");
    }
    if args.tomato_meter {
        props.push("tomatoMeter");
    }
    if args.box_office {
        props.push("BoxOffice");
    }
    if args.rated {
        props.push("Rated");
    }
    if args.awards {
        props.push("Awards");
    }
    if args.poster {
        props.push("Poster");
    }

    props
}

use colored::*;
use serde_json::Value;
/// Print formatted film or series information.
///
/// If the OMDb API returns an error, a red error will be displayed and the progam will exit.
///
/// # Arguments
///
/// * `media` - JSON response from OMDb.
/// * `props_to_show` - Vector of properties to display.
fn print_info(media: &Value, props_to_show: Vec<&'static str>) {
    if media["Response"] == "False" {
        let error_msg = media["Error"].as_str().unwrap_or("Unknown error.");
        eprintln!("{}", error_msg.red());
        std::process::exit(1);
    }

    for prop in props_to_show {
        if let Some(val) = media[prop].as_str()
            && val != "N/A"
        {
            let spacing = 13usize.saturating_sub(prop.len());
            println!("{}{} :: {}", prop.bold().green(), " ".repeat(spacing), val)
        }
    }
}

/// Entry point
///
/// Parses CLI arguments, requests response from OMDb, and prints requested information.
fn main() {
    let args = Reel::parse();
    if args.query.is_empty() {
        eprintln!("Please provide a film or series name.");
        std::process::exit(1);
    } else {
        let title = args.query.replace(' ', "+");
        let mut url = format!("{}{}&t={}", API_URL, DEFAULT_KEY, title);

        if let Some(year) = args.year {
            url.push_str(&format!("&y={}", year));
        }

        let response: Value = reqwest::blocking::get(&url)
            .expect("Failed to make request.")
            .json::<Value>()
            .expect("Failed to parse JSON.");

        let props_to_show = build_props(&args);

        print_info(&response, props_to_show);
    }
}
