//! tests/subscription.rs

use email_newsletter::configuration::get_configuration;
use email_newsletter::utils::spawn_sut;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let sut_address = spawn_sut();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to the database");

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

    let saved = sqlx::query!("SELECT name, email FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "John Doe");
    assert_eq!(saved.email, "john.doe@example.com");
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    let sut_address = spawn_sut();
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
