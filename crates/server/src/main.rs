use configuration::Settings;
use server::app;
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let settings = Settings::load().expect("Failed to read configuration");
    let listener = TcpListener::bind(settings.address()).expect("Failed to bind port");

    println!("{}", &BANNER_TEXT);
    tracing::info!("Starting the server...");
    tracing::info!("{}", serde_json::to_string(&settings).unwrap());

    app::run(listener, &settings).await;
}

const BANNER_TEXT: &str = r"
 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/
";
