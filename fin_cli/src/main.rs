mod config;

use clap::{Parser, Subcommand, ValueEnum};

use config::*;
use fin::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        path: String,
        #[arg(value_enum)]
        variant: Variant,
    },
    ClassifyAll,
    ClassifyUncategorised,
}

#[derive(ValueEnum, Clone)]
enum Variant {
    ING,
    Bendigo,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = Config::from_env();
    let service = Service::new(&config.database_url)
        .await
        .expect("Failed to create service");

    match cli.cmd {
        Command::Add { path, variant } => {
            todo!()
        }
        Command::ClassifyAll => {
            service
                .classify_all_transactions()
                .await
                .expect("Failed to classify transactions");
        }
        Command::ClassifyUncategorised => {
            service
                .classify_uncategorised_transactions()
                .await
                .expect("Failed to classify transactions");
        }
    }
}
