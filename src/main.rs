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
    #[arg(required = true, num_args = 1..=2, help = "Film or series title.")]
    query: Vec<String>,

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

use reqwest::Client;
use serde_json::Value;
/// Fetches film or series information from the OMDb API asynchronously.
///
/// # Arguments
///
/// * `client` - A reference to a `reqwest::Client` to reuse connections efficiently.
/// * `title` - The title of the movie or series to fetch.
/// * `year` - Optional release year to narrow the search.
async fn fetch_movie_async(client: &Client, title: &str, year: Option<u16>) -> Result<Value, String> {
    let formatted = title.replace(' ', "+");
    let mut url = format!("{}{}&t={}", API_URL, DEFAULT_KEY, formatted);

    if let Some(y) = year {
        url.push_str(&format!("&y={}", y));
    }

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| format!("Failed to send request to '{}'", url))?;

    if !response.status().is_success() {
        return Err(format!(
            "Film/series '{}' not found (HTTP {})",
            title,
            response.status()
        ));
    }

    response
        .json::<Value>()
        .await
        .map_err(|_| "Failed to parse response JSON.".to_string())
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
    let base = ["Title", "Director", "Year", "Runtime", "Genre"];
    let flags = [
        (args.writer, "Writer"),
        (args.released, "Released"),
        (args.actors, "Actors"),
        (args.plot, "Plot"),
        (args.language, "Language"),
        (args.country, "Country"),
        (args.metascore, "Metascore"),
        (args.imdb_rating, "imdbRating"),
        (args.box_office, "BoxOffice"),
        (args.rated, "Rated"),
        (args.awards, "Awards"),
        (args.poster, "Poster"),
    ];

    let mut props: Vec<&'static str> = base.into();
    props.extend(
        flags
            .into_iter()
            .filter_map(|(flag, prop)| flag.then_some(prop)),
    );

    props
}

use colored::*;
/// Print formatted film or series information.
///
/// If the OMDb API returns an error, a red error will be displayed and the progam will exit.
///
/// # Arguments
///
/// * `media` - JSON response from OMDb.
/// * `props_to_show` - Vector of properties to display.
fn print_info(media: &Value, props: Vec<&'static str>) {
    if media["Response"] == "False" {
        let error_msg = media["Error"].as_str().unwrap_or("Unknown error.");
        eprintln!("{}", error_msg.red());
        std::process::exit(1);
    }

    for prop in props {
        if let Some(val) = media[prop].as_str()
            && val != "N/A"
        {
            let spacing = 13usize.saturating_sub(prop.len());
            println!("{}{} {}", prop.bold().green(), " ".repeat(spacing), val)
        }
    }
}

/// Wrap a string to a fixed size in the terminal.
///
/// * `s` - String to wrap.
/// * `width` - Size to wrap to.
fn wrap_lines(s: &str, width: usize) -> Vec<String> {
    textwrap::wrap(s, width)
        .into_iter()
        .map(|line| line.to_string())
        .collect()
}

/// Print two films/series next to each other.
///
/// * `a` - First film/series.
/// * `b` - Second film/series.
/// * `props` - Properties to display.
fn print_side_by_side(a: &Value, b: &Value, props: Vec<&'static str>) {
    let col_width = 40;

    println!("{}", "Comparison".bold().yellow());

    for prop in props {
        let left_raw = a[prop].as_str().unwrap_or("N/A");
        let right_raw = b[prop].as_str().unwrap_or("N/A");

        if prop == "Poster" {
            continue;
        }

        let left_lines = wrap_lines(left_raw, col_width);
        let right_lines = wrap_lines(right_raw, col_width);

        let max = left_lines.len().max(right_lines.len());

        for i in 0..max {
            let left = left_lines.get(i).map(|s| s.as_str()).unwrap_or("");
            let right = right_lines.get(i).map(|s| s.as_str()).unwrap_or("");

            if i == 0 {
                println!("{:<15} {:<40} {}", prop.green(), left, right);
            } else {
                println!("{:<15} {:<40} {}", "", left, right);
            }
        }
    }
}

/// Print the difference in specific stats between two films/series.
///
/// * `a` - First film/series.
/// * `b` - Second film/series.
fn print_difference(a: &Value, b: &Value) {
    println!("\n{}", "Differences".bold().yellow());

    fn parse_num(s: &str) -> Option<f64> {
        s.replace("min", "")
            .replace("$", "")
            .replace(",", "")
            .trim()
            .parse::<f64>()
            .ok()
    }

    let fields = [
        ("Runtime", "min"),
        ("Metascore", " pts"),
        ("imdbRating", " pts"),
        ("BoxOffice", " USD"),
    ];

    for (field, unit) in fields {
        let va = a[field].as_str().unwrap_or("N/A");
        let vb = b[field].as_str().unwrap_or("N/A");

        if let (Some(n1), Some(n2)) = (parse_num(va), parse_num(vb)) {
            let diff = n1.max(n2) - n1.min(n2);
            println!(
                "{:<15} {}{} ({:+.2} {})",
                field.green(),
                n1,
                unit,
                diff,
                unit
            );
        }
    }
}

/// Entry point for reel CLI
#[tokio::main]
async fn main() {
    let args = Reel::parse();

    if args.query.is_empty() || args.query.iter().any(|q| q.trim().is_empty()) {
        eprintln!("{}", "Please provide non-empty title(s).".red());
        std::process::exit(1);
    }

    let client = reqwest::Client::new();

    if args.query.len() == 1 {
        let movie1 = match fetch_movie_async(&client, &args.query[0], args.year).await {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", e.red());
                std::process::exit(1);
            }
        };

        let props_to_show = build_props(&args);
        print_info(&movie1, props_to_show);

    } else {
        // Parallel fetch both movies
        let (movie1_res, movie2_res) = tokio::join!(
            fetch_movie_async(&client, &args.query[0], args.year),
            fetch_movie_async(&client, &args.query[1], args.year)
        );

        let movie1 = match movie1_res {
            Ok(m) => m,
            Err(e) => { eprintln!("{}", e.red()); std::process::exit(1); }
        };
        let movie2 = match movie2_res {
            Ok(m) => m,
            Err(e) => { eprintln!("{}", e.red()); std::process::exit(1); }
        };

        let props_to_show = build_props(&args);
        print_side_by_side(&movie1, &movie2, props_to_show);
        print_difference(&movie1, &movie2);
    }
}
