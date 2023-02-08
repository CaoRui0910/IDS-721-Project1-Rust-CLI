/*Command-line interface for Horror Movies */
use clap::Parser;
use std::env;
use std::error::Error;
use std::io;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Command-line interface for Horror Movies"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    GetMovieInfo{
        movie_name: String,
    },
    GetPopularMovie{
        popularity: f64,
    },
    GetMovieByRevenue{
        revenue: u64,
    },
    GetMoviesByVote{
        vote_average: f64,
    },
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    id: Option<String>,
    original_title: Option<String>,
    title: String,
    original_language: Option<String>,
    overview: Option<String>,
    tagline: Option<String>,
    release_date: Option<String>,
    poster_path: Option<String>,
    popularity: Option<f64>,
    vote_count: Option<u64>,
    vote_average: Option<f64>,
    budget: Option<f64>,
    revenue: Option<u64>,
    runtime: Option<u64>,
    status: Option<String>,
    adult: Option<String>,
    backdrop_path: Option<String>,
    genre_names: Option<String>,
    collection: Option<String>,
    collection_name: Option<String>,
}

fn filter_by_title(movie_name: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.title == movie_name {
            wtr.serialize(record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn filter_by_revenue(revenue: u64) -> Result<(), Box<dyn Error>> {
    let minimum_rev: u64 = revenue;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.revenue.map_or(false, |revenue| revenue >= minimum_rev) {
            wtr.serialize(record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn filter_by_popularity(popularity: f64) -> Result<(), Box<dyn Error>> {
    let minimum_popularity: f64 = popularity;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.popularity.map_or(false, |popularity| popularity >= minimum_popularity) {
            wtr.serialize(record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn filter_by_vote(vote: f64) -> Result<(), Box<dyn Error>> {
    let minimum_vote: f64 = vote;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.vote_average.map_or(false, |vote_average| vote_average >= minimum_vote) {
            wtr.serialize(record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::GetMovieInfo{ movie_name }) => {
            if let Err(err) = filter_by_title(movie_name){
                println!("{}", err);
                process::exit(1);
            }
        }
        Some(Commands::GetPopularMovie{ popularity }) => {
            if let Err(err) = filter_by_popularity(popularity){
                println!("{}", err);
                process::exit(1);
            }
        }
        Some(Commands::GetMovieByRevenue{ revenue }) => {
            if let Err(err) = filter_by_revenue(revenue){
                println!("{}", err);
                process::exit(1);
            }
        }
        Some(Commands::GetMoviesByVote{ vote_average }) => {
            if let Err(err) = filter_by_vote(vote_average){
                println!("{}", err);
                process::exit(1);
            }
        }
        None => println!("No subcommand was used"),
    }
}
