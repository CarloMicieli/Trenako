use crate::{catalog, health_check, json_configuration};
use actix_web::dev::Server;
use actix_web::middleware::{Compress, DefaultHeaders};
use actix_web::{web, App, HttpServer};
use configuration::Settings;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Run the web server
pub fn run(listener: TcpListener, settings: &Settings) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(settings.database.get_connection_pool());

    #[rustfmt::skip]
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Compress::default())
            .wrap(default_headers())
            .route("/health-check", web::get().to(health_check::handler))
            .configure(catalog::config_services)
            .app_data(json_configuration::json_config())
            .app_data(db_pool.clone())
        })
        .workers(settings.workers())
        .listen(listener)?
        .run();
    Ok(server)
}

fn default_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        //Set to "nosniff" to prevent the browser guessing the correct Content-Type.
        .add(("X-Content-Type-Options", "nosniff"))
}
