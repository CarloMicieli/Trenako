use sqlx::PgPool;
use std::net::TcpListener;
use web::configuration::Settings;
use web::server;
use web::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("trenako".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = Settings::load().expect("Failed to read configuration");
    let listener = TcpListener::bind(config.address()).expect("Failed to bind port");
    let db_pool = PgPool::connect(&config.database_url())
        .await
        .expect("Failed to connect to Postgres.");

    println!("{}", &BANNER_TEXT);
    println!("Starting the server ({})...", config.address());
    server::run(listener, db_pool, config.workers())?.await
}

const BANNER_TEXT: &str = r#"
 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/
"#;
