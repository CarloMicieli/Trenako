pub mod cli_parser;
pub mod schemas;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Validate,
    Seed,
}
