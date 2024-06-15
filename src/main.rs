use std::net::TcpListener;

use bag_of_holding::{configuration::get_configuration, startup::run};
use env_logger::Env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to Postgres.");
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", configuration.application_port))
        .expect("failed to bind to address");
    run(listener, connection_pool)?.await
}
