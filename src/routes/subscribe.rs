//! src/routes/subscribe.rs

use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct SubscriptionData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<SubscriptionData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            // Subscription successful
            HttpResponse::Created().finish()
        }
        Err(e) => {
            // Handle error, e.g., log it
            eprintln!("Failed to insert subscription: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
