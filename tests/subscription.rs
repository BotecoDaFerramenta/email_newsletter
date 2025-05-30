//! tests/subscription.rs

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let sut_address = email_newsletter::spawn_sut();
    let client = reqwest::Client::new();

    let body = "name=John%20Doe&email=john.doe%40example.com";
    let response = client
        .post(&format!("{}/subscribe", &sut_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    let sut_address = email_newsletter::spawn_sut();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=John%20Doe", "Missing email"),
        ("email=john.doe%40example.com", "Missing name"),
        ("", "Missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &sut_address))
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
