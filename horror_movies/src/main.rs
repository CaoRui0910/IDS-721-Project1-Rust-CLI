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
    Infer {},
}


fn main() {
    println!("Hello, world!");
}
