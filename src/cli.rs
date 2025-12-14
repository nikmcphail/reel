use clap::Parser;

#[derive(Debug, Parser)]
#[command(about)]
pub struct Reel {
    #[arg(required = true, num_args = 1.., help = "Film or series title.")]
    pub queries: Vec<String>,

    #[arg(
        short = 'y',
        long = "year",
        help = "Release year. (Only works for single title.)"
    )]
    pub year: Option<u16>,

    #[arg(short = 'w', long = "writer", help = "Show the writer(s).")]
    pub writer: bool,
    #[arg(short = 'r', long = "released", help = "Show release date.")]
    pub released: bool,
    #[arg(short = 'a', long = "actors", help = "Show the main cast.")]
    pub actors: bool,
    #[arg(short = 'l', long = "language", help = "Show the language(s).")]
    pub language: bool,
    #[arg(short = 'c', long = "country", help = "Show the country(ies).")]
    pub country: bool,
    #[arg(short = 'm', long = "metascore", help = "Show the Metacritic score.")]
    pub metascore: bool,
    #[arg(short = 'i', long = "imdb", help = "Show the IMDb rating.")]
    pub imdb: bool,
    #[arg(
        short = 'b',
        long = "box-office",
        help = "Show the box office earnings."
    )]
    pub box_office: bool,
    #[arg(short = 'R', long = "rating", help = "Show the MPA rating.")]
    pub rating: bool,
    #[arg(short = 'A', long = "awards", help = "Show award nominations and wins.")]
    pub awards: bool,
    #[arg(short = 'p', long = "plot", help = "Show the plot summary.")]
    pub plot: bool,
}
