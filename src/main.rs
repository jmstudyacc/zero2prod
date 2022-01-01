//! src/main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // '?' added to allow for an Error to be passed to main()
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
