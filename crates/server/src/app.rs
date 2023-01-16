use crate::catalog;
use crate::configuration::Settings;
use actix_web::dev::Server;
use actix_web::middleware::Compress;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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
            .route("/health-check", web::get().to(health_check))
            .configure(catalog::config_services)
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(db_pool.clone())
        })
        .workers(settings.workers())
        .listen(listener)?
        .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
