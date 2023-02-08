# Filter for horror movies
## Intrucrion
This project is a command line tool completed with Rust. It filters out films that meet their requirements for horror film lovers. It is important that the dataset of this project contains most of the horror films in the world since the 1950s, so horror movie lovers can't miss this tool!

## Dataset
- The dataset (horror_movies.csv) of this project is from kaggle (https://www.kaggle.com/datasets/evangower/horror-movies).
- This horror movies dataset includes films dating back to the 1950s. There are ~32K movie records in this dataset.
- Note: Because the original dataset is too large, the running time of the program will become longer. I used a smaller dataset (small_horror_movies.csv) just for the purpose of testing. 

## Usage
- User can type in `cargo run -- subcommand -- commandLineArg < small_horror_movies.csv` in command line.
- Subcommands in this project:
  - get-movie-info: Filter out all movies with the movie name "commandLineArg", and output all details of these movies.
  - get-popular-movie: Filter out all movies with more than "commandLineArg" popularity, and output all details of these movies.
  - get-movie-by-revenue: Filter out all movies with more than "commandLineArg" revenue, and output all details of these movies.
  - get-movies-by-vote: Filter out all movies with more than "commandLineArg" average vote, and output all details of these movies.
- Here are some examples:
    ```
    cargo run -- get-movie-info -- Jeepers Creepers: Reborn < small_horror_movies.csv
    cargo run -- get-popular-movie -- 100 < small_horror_movies.csv
    cargo run -- get-movie-by-revenue -- 40000 < small_horror_movies.csv
    cargo run -- get-movies-by-vote -- 7 < small_horror_movies.csv
    ```
The output of `cargo run -- get-popular-movie -- 100 < small_horror_movies.csv` is:
<img width="933" alt="Screen Shot 2023-02-08 at 01 27 49" src="https://user-images.githubusercontent.com/93239143/217451692-75edfe61-56ab-4dd0-9f38-231494b343d3.png">

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
