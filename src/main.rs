use std::net::TcpListener;
use sqlx::PgPool;
use rust_microservice::configuration::get_configuration;
use rust_microservice::startup;
use rust_microservice::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust-microservice".into(), "info".into());
    init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await
}