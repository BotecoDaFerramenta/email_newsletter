//! src/routes/subscribe.rs

use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{Instrument, error, info_span};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscriptionData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<SubscriptionData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = info_span!("Saving new subscriber in the database");

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
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            error!(
                "request_id {} - Failed to insert subscription: {:?}",
                request_id, e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
