/*Command-line interface for Horror Movies */
use clap::Parser;

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
        movie_name: String,
    },
    GetMovieByRevenue{
        movie_name: String,
    },
    GetMoviesByVote{
        movie_name: String,
    },
}


fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::GetMovieInfo{ movie_name }) => {
            let result = movie_name;
            print!("Hello,");
            println!("{}", result);
        }
        Some(Commands::GetPopularMovie{ movie_name }) => {
            let result = movie_name;
            print!("Bye,");
            println!("{}", result);
        }
        Some(Commands::GetMovieByRevenue{ movie_name }) => {
            let result = movie_name;
            print!("B-B,");
            println!("{}", result);
        }
        Some(Commands::GetMoviesByVote{ movie_name }) => {
            let result = movie_name;
            print!("B-B-B,");
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
