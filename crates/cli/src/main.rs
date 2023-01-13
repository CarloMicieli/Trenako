use cli::dataset::Dataset;
use cli::validator::validate_dataset;
use cli::{cli_parser, Command};
use serde_json::json;

fn main() {
    let command = cli_parser::parse();
    match command {
        Some(Command::Validate(s)) => {
            let result = Dataset::from_path(&s);
            match result {
                Ok(dataset) => {
                    println!("{}", dataset);
                    let result = validate_dataset(dataset).unwrap();

                    for v in result.iter().filter(|x| !x.is_valid()) {
                        println!("{:#?}", json!(v));
                    }
                }
                Err(why) => why.display(),
            }
        }
        Some(Command::Seed) => {}
        _ => {}
    }
}
