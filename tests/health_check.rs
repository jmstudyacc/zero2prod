//! tests/health_check.rs

use zero2prod::run;
// Before enabling this test file the work needs to be refactored into a library and a binary
// [tokio::test] is the testing equivalent to [tokio::main]
#[tokio::test]
async fn health_check_success() {
    // Arrange
    let address = spawn_app(); // no .await or .expect needed

    // import reqwest to run HTTP Requests against the client
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=james%20miles&email=jmstudyacc%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    // At present this assertion does not tell us the business purpose has been achieved
    assert_eq!(200, response.status().as_u16());
}

// Below test is an example of a table-driven or parameterised test
#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=james&20miles", "missing the email"),
        ("email=jmstudyacc%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // customised error message
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// Launch application in the background
fn spawn_app() -> String {
    // using 127.0.0.1:0 for testing will use any random port to complete the testing
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");

    // provides the random port provided via OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address.");
    // launch the server as a background task
    // tokio::spawn will return a handle to the spawned future but we do not use here
    let _ = tokio::spawn(server);

    // formats the target URL
    format!("http://127.0.0.1:{}", port)
}
