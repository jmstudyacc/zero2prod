//! tests/health_check.rs

// Before enabling this test file the work needs to be refactored into a library and a binary
// [tokio::test] is the testing equivalent to [tokio::main]
#[tokio::test]
async fn health_check_success() {
    // Arrange
    spawn_app(); // no .await or .expect needed

    // import reqwest to run HTTP Requests against the client
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address.");
    // launch the server as a background task
    // tokio::spawn will return a handle to the spawned future but we do not use here
    let _ = tokio::spawn(server);
}
