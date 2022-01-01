//! src/main.rs

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // '?' added to allow for an Error to be passed to main()
    run()?.await
}
