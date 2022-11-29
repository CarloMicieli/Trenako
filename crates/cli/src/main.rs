#[allow(dead_code)]
mod data;

use clap::Command;

#[macro_use]
extern crate serde_derive;

fn main() {
    let _matches = Command::new("trenako")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Model railway collection manager")
        .author(env!("CARGO_PKG_AUTHORS"))
        .get_matches();
}
