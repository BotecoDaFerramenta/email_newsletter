//! main.rs

use email_newsletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:1127").expect("Failed to bind random port");
    run(listener)?.await
}
