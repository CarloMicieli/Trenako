use cli::dataset::Dataset;
use cli::validator::validate_dataset;
use cli::{cli_parser, Command};

fn main() {
    let command = cli_parser::parse();
    match command {
        Some(Command::Validate(s)) => {
            let result = Dataset::from_path(&s);
            match result {
                Ok(dataset) => {
                    println!("{}", dataset);
                    validate_dataset(dataset).unwrap();
                }
                Err(why) => why.display(),
            }
        }
        Some(Command::Seed) => {}
        _ => {}
    }
}
