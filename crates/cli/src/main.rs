use cli::cli_parser;

fn main() {
    let command = cli_parser::parse();
    println!("{:?}", command);
}
