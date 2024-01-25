mod adapter;
mod config;

use clap::{Parser, Subcommand, ValueEnum};

use adapter::*;
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
        Command::Add { path, variant: _ } => {
            let is = get_ing_transactions(&path);
            let rs = service.create_transactions(is).await;
            println!("{:?}", rs)
        }
        Command::ClassifyAll => {
           let updates = service
                .classify_all_transactions()
                .await
                .expect("Failed to classify transactions");
            println!("{:#?}", updates);
        }
        Command::ClassifyUncategorised => {
            service
                .classify_uncategorised_transactions()
                .await
                .expect("Failed to classify transactions");
        }
    }
}
