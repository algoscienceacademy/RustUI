use clap::{Parser, Subcommand};
use rust_native::dev_server::DevServer;

#[derive(Parser)]
#[command(name = "rust-native")]
#[command(about = "RustUI Native Development Tools")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,  // Make Commands required by removing Option
}

#[derive(Subcommand)]
enum Commands {
    /// Start the development server
    Dev {
        /// Project path to watch
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dev { path } => {
            println!("Starting development server in: {}", path);
            let mut server = DevServer::new();
            server.watch(&path)?;
            server.rebuild(); // Start initial build
            server.run()
        }
    }
}
