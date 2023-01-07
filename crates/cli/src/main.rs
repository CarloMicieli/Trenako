use clap::Command;

fn main() {
    let _matches = Command::new("trenako-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about(BANNER_TEXT)
        .author(env!("CARGO_PKG_AUTHORS"))
        .get_matches();
}

const BANNER_TEXT: &str = r#"
 _                        _                   _ _
| |                      | |                 | (_)
| |_ _ __ ___ _ __   __ _| | _____ ______ ___| |_
| __| '__/ _ \ '_ \ / _` | |/ / _ \______/ __| | |
| |_| | |  __/ | | | (_| |   < (_) |    | (__| | |
 \__|_|  \___|_| |_|\__,_|_|\_\___/      \___|_|_|

> A command line tool on top of trenako.com
"#;
