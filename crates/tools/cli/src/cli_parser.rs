use crate::Command;
use clap::{Parser, Subcommand};

const BANNER_TEXT: &str = r"
 _                        _                   _ _
| |                      | |                 | (_)
| |_ _ __ ___ _ __   __ _| | _____ ______ ___| |_
| __| '__/ _ \ '_ \ / _` | |/ / _ \______/ __| | |
| |_| | |  __/ | | | (_| |   < (_) |    | (__| | |
 \__|_|  \___|_| |_|\__,_|_|\_\___/      \___|_|_|

> A command line tool on top of trenako.com
";

#[derive(Parser)]
#[command(name = "trenako-cli")]
#[command(about = "A command line tool on top of trenako.com", long_about = Some(BANNER_TEXT))]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// validate the dataset
    Validate {
        /// Name of the person to greet
        #[arg(short, long)]
        source: String,
    },

    /// seed the application database
    Seed {},
}

pub fn parse() -> Option<Command> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Validate { source }) => Some(Command::Validate(source.clone())),
        Some(Commands::Seed {}) => Some(Command::Seed),
        None => None,
    }
}
