use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap the db-pool in a smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Use app_data to append application state
            .app_data(db_pool.clone())
        // Use wrap to add any middleware
            .wrap(Logger::default())
    })
        .listen(listener)?
        .run();
    Ok(server)
}