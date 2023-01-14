use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use web::configuration::Settings;
use web::server;
use web::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("trenako".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::load().expect("Failed to read configuration");
    let listener = TcpListener::bind(settings.address()).expect("Failed to bind port");
    let db_pool = get_connection_pool(&settings);

    println!("{}", &BANNER_TEXT);
    println!("Starting the server ({})...", settings.address());
    server::run(listener, db_pool, settings.workers())?.await
}

fn get_connection_pool(configuration: &Settings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.pg_connection_options())
}

const BANNER_TEXT: &str = r#"
 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/
"#;
