use crate::{Command, Mode};
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
    /// exports the catalog items to a csv file
    CsvExport {
        /// the source directory
        #[arg(short, long)]
        source: String,

        #[arg(short, long)]
        output: String,

        #[arg(short, long)]
        dry_run: bool,
    },

    /// imports catalog items from a csv file
    CsvImport {
        /// the name of the csv file to import
        #[arg(short, long)]
        file: String,

        /// the import mode
        #[arg(short, long)]
        mode: Mode,

        #[arg(short, long)]
        output: String,

        #[arg(short, long)]
        dry_run: bool,
    },

    /// seed the application database
    Seed {},

    /// validate the dataset
    Validate {
        /// the source directory
        #[arg(short, long)]
        source: String,
    },
}

pub fn parse() -> Option<Command> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::CsvExport {
            source,
            output,
            dry_run,
        }) => Some(Command::CsvExport {
            source: source.clone(),
            output: output.clone(),
            dry_run: *dry_run,
        }),
        Some(Commands::CsvImport {
            file,
            mode,
            output,
            dry_run,
        }) => Some(Command::CsvImport {
            file: file.clone(),
            mode: *mode,
            output: output.clone(),
            dry_run: *dry_run,
        }),
        Some(Commands::Seed {}) => Some(Command::Seed),
        Some(Commands::Validate { source }) => Some(Command::Validate(source.clone())),
        None => None,
    }
}
