//! tests/health_check.rs

use email_newsletter::utils::spawn_sut;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_sut().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
