//! tests/subscription.rs

use email_newsletter::utils::spawn_sut;

#[tokio::test]
async fn subscribe_return_a_201_for_valid_form_data() {
    let app = spawn_sut().await;
    let client = reqwest::Client::new();

    let body = "name=John%20Doe&email=john.doe%40example.com";
    let response = client
        .post(&format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "John Doe");
    assert_eq!(saved.email, "john.doe@example.com");
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    let app = spawn_sut().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=John%20Doe", "Missing email"),
        ("email=john.doe%40example.com", "Missing name"),
        ("", "Missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Expected 400 for: {}",
            error_message
        );
    }
}
