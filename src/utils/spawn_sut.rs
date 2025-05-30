//! src/utils/test.rs

use crate::configuration::get_configuration;
use crate::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_sut() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database");
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);
    TestApp {
        address: address.clone(),
        db_pool,
    }
}
