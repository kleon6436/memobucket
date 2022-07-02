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
    #[clap(about = "Add memo to file.")]
    Add { message: Option<String> },

    // Read memo from file.
    #[clap(about = "Read memo from file.")]
    Read { file_path: Option<String> },

    // Show memo
    #[clap(about = "Show memo.")]
    Show,

    // Remove memo file
    #[clap(about = "Remove memo file. Please put memo file path in command arguments.")]
    Remove { file_path: Option<String> },
}

// Log file path
const LOG_FILE_PATH: &str = "memobucket.md";

fn main() {
    let args = Args::parse();
    let log_file_manager = log_file_manager::LogFileManager::new(LOG_FILE_PATH);

    match &args.commands {
        Commands::Add { message } => {
            if let Some(message) = message {
                log_file_manager.write_log(message)
            } else {
                println!("Please set some message.")
            };
        }
        Commands::Read { file_path } => {
            if let Some(file_path) = file_path {
                log_file_manager.read_log(file_path);
            } else {
                println!("Log file is not found.");
                println!("Please put read log file path in command line arguments.");
            }
        }
        Commands::Show => {
            log_file_manager.show_log();
        }
        Commands::Remove { file_path } => {
            if let Some(file_path) = file_path {
                log_file_manager.remove_log_file(file_path);
            } else {
                println!("Log file is not found.");
                println!("Please put delte log file path in command line arguments.");
            }
        }
    }
}
