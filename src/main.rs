const API_URL: &str = "http://www.omdbapi.com/?apikey=";
const DEFAULT_KEY: &str = "5e540903";

use clap::Parser;
#[derive(Parser)]
#[command(about)]
struct Reel {
    query: String,

    #[arg(short = 'y', long = "year", help = "Specify release year")]
    year: Option<u16>,

    #[arg(short = 'w', long = "writer", help = "Show the writer(s)")]
    writer: bool,
    #[arg(short = 'r', long = "released", help = "Show the release date")]
    released: bool,
    #[arg(short = 'a', long = "actors", help = "Show the main cast")]
    actors: bool,
    #[arg(short = 'p', long = "plot", help = "Show the plot summary")]
    plot: bool,
    #[arg(short = 'l', long = "language", help = "Show the language(s)")]
    language: bool,
    #[arg(
        short = 'c',
        long = "country",
        help = "Show the country(ies) of production"
    )]
    country: bool,
    #[arg(short = 'm', long = "metascore", help = "Show the Metacritic score")]
    metascore: bool,
    #[arg(short = 'i', long = "imdb-rating", help = "Show the IMDb rating")]
    imdb_rating: bool,
    #[arg(
        short = 't',
        long = "tomato-meter",
        help = "Show the Rotten Tomatoes score"
    )]
    tomato_meter: bool,
    #[arg(
        short = 'b',
        long = "box-office",
        help = "Show the box office earnings"
    )]
    box_office: bool,
    #[arg(short = 'R', long = "rated", help = "Show the MPA rating")]
    rated: bool,
    #[arg(short = 'A', long = "awards", help = "Show award wins and nominations")]
    awards: bool,
}

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

    props
}

use colored::*;
use serde_json::Value;
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
