use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;

/// Run the web server
pub fn run(listener: TcpListener, db_pool: PgPool, workers: usize) -> Result<Server, std::io::Error> {
    #[rustfmt::skip]
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
        })
        .workers(workers)
        .listen(listener)?
        .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
