pub mod cli_parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Validate,
    Seed,
}
