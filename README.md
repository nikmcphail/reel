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
<img width="404" height="376" alt="image" src="https://github.com/user-attachments/assets/01efa68f-d349-412a-abb0-b8750da3370b" />

## Example Outputs
`reel "La La Land" -waiA`

<img width="450" height="153" alt="image" src="https://github.com/user-attachments/assets/0baa78ac-f9d5-45fc-b724-996849c30dbc" />

`reel "Project X" -y 1987 -Rm`

<img width="276" height="121" alt="image" src="https://github.com/user-attachments/assets/71df6cc9-bcb6-4784-9336-2afb2417b1c7" />

## License
This project is released under the [Unlicense](LICENSE). You can copy, modify, and distribute it freely.
