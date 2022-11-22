use clap::Command;

fn main() {
    let matches = Command::new("trenako")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Model railway collection manager")
        .author(env!("CARGO_PKG_AUTHORS"))
        .get_matches();

    match matches.subcommand() {
        _ => print!("nope"),
    }
}
