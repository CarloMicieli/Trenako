use server::app;
use server::configuration::Settings;
use server::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("trenako".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::load().expect("Failed to read configuration");
    let listener = TcpListener::bind(settings.address()).expect("Failed to bind port");

    println!("{}", &BANNER_TEXT);
    tracing::info!("Starting the server...");
    tracing::info!("{}", serde_json::to_string(&settings).unwrap());
    app::run(listener, &settings)?.await
}

const BANNER_TEXT: &str = r#"
 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/
"#;
