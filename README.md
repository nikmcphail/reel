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

<img width="425" height="152" alt="image" src="https://github.com/user-attachments/assets/61105a26-b088-4681-864d-1a78593c0515" />

`reel "Project X" -y 1987 -Rm`

<img width="267" height="121" alt="image" src="https://github.com/user-attachments/assets/986c528e-27d7-40bf-809e-7d4646e1faac" />

`reel "Snowpiercer" "Scott Pilgrim vs the World" -b`

<img width="595" height="222" alt="image" src="https://github.com/user-attachments/assets/865fff1a-a7e9-4401-b7bc-4109907408fd" />

## License
This project is released under the [Unlicense](LICENSE). You can copy, modify, and distribute it freely.
