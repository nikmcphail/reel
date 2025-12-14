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

<img width="468" height="298" alt="image" src="https://github.com/user-attachments/assets/279beb10-5d6e-4f91-ac04-7d0e4f9deceb" />

## Example Outputs
`reel "La La Land" -wai`

<img width="1051" height="84" alt="image" src="https://github.com/user-attachments/assets/78865ab6-e06c-4587-8ff5-7a0234fa509b" />

`reel "Project X" -y 1987 -Rm`

<img width="685" height="80" alt="image" src="https://github.com/user-attachments/assets/93668047-e49c-45fb-a868-63686088f530" />

`reel "Snowpiercer" "Scott Pilgrim vs the World" -b`

<img width="735" height="115" alt="image" src="https://github.com/user-attachments/assets/498ad093-30d3-4dae-a976-b54e33b9e3a8" />

## License
This project is released under the [Unlicense](LICENSE). You can copy, modify, and distribute it freely.
