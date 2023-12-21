use configuration::Settings;
use server::app;
use server::tracing::init_tracing;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let settings = Settings::load().expect("Failed to read configuration");
    init_tracing(&settings.logging);

    let listener = TcpListener::bind(settings.address())
        .await
        .expect("Failed to bind port");

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
