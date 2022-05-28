use clap::{Parser, Subcommand};
use memobucket::log_file_manager;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Add memo to memo file.
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Add memo to file.
    #[clap(about="Add memo to file.")]
    Add { message: Option<String> },

    // Read memo from file.
    #[clap(about="Read memo from file.")]
    Read { file_path: Option<String> },

    // Show memo
    #[clap(about="Show memo.")]
    Show,
}

fn main() {
    let args = Args::parse();

    match &args.commands {
        Commands::Add { message } => {
            let log_file_manager = log_file_manager::LogFileManager::new("test.txt");
            
            if let Some(message) = message {
                log_file_manager.write_log(message)
            } else {
                println!("Please set some message.")
            };
        }
        Commands::Read { file_path } => {
            println!("FilePath: {:?}", &file_path)
        }
        Commands::Show => {
            println!("Show memo")
        }
    }
}
