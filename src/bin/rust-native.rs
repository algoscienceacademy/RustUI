use clap::{Parser, Subcommand};
use rust_native::dev_server::DevServer;

#[derive(Parser)]
#[command(name = "rust-native")]
#[command(about = "RustUI Native Development Tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Dev {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev { path } => {
            let mut server = DevServer::new();
            server.watch(&path)?;
            server.run()?;
        }
    }

    Ok(())
}
