# reel

**reel** — CLI written in Rust to search up information about films or series.

## What is this?
`reel` lets you search for movie or series information using the command line.
Inspired by mayankchd's [movie](https://github.com/mayankchd/movie) CLI.

## Features
- Search for movies/series by title
- Choose information you want to see using command line arguments
- Specify release year for search.

## Prerequisites
- Rust

## Build
```bash
git clone https://github.com/nikmcphail/reel.git
cd reel
cargo build --release
```

## Arguments / Help

```
Usage: reel [OPTIONS] <QUERIES>...

Arguments:
  <QUERIES>...  Film or series title.

Options:
  -y, --year <YEAR>  Release year. (Only works for single title.)
  -w, --writer       Show the writer(s).
  -r, --released     Show release date.
  -a, --actors       Show the main cast.
  -l, --language     Show the language(s).
  -c, --country      Show the country(ies).
  -m, --metascore    Show the Metacritic score.
  -i, --imdb         Show the IMDb rating.
  -b, --box-office   Show the box office earnings.
  -R, --rating       Show the MPA rating.
  -A, --awards       Show award nominations and wins.
  -p, --plot         Show the plot summary.
  -C, --compare      Compare titles.
  -h, --help         Print help
  ```

## Example Outputs
`reel "La La Land" -wai`

<img width="1051" height="84" alt="image" src="https://github.com/user-attachments/assets/78865ab6-e06c-4587-8ff5-7a0234fa509b" />

`reel "Project X" -y 1987 -Rm`

<img width="685" height="80" alt="image" src="https://github.com/user-attachments/assets/93668047-e49c-45fb-a868-63686088f530" />

`reel "Snowpiercer" "Scott Pilgrim vs the World" -b`

<img width="735" height="115" alt="image" src="https://github.com/user-attachments/assets/498ad093-30d3-4dae-a976-b54e33b9e3a8" />

## License
This project is released under the [Unlicense](LICENSE). You can copy, modify, and distribute it freely.
