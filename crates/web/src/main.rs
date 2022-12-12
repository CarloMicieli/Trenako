use std::net::TcpListener;
use web::configuration::Settings;
use web::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::load().expect("Failed to read configuration");
    let listener = TcpListener::bind(config.address()).expect("Failed to bind port");

    println!("{}", &BANNER_TEXT);
    println!("Starting the server ({})...", config.address());
    run(listener)?.await
}

const BANNER_TEXT: &str = r#"
 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/
"#;
