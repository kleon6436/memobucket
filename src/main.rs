use clap::{Parser, Subcommand};

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
            println!("test: {:?}", &message)
        }
        Commands::Read { file_path } => {
            println!("FilePath: {:?}", &file_path)
        }
        Commands::Show => {
            println!("Show memo")
        }
    }
}
