//! src/routes/subscribe.rs

use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct SubscriptionData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<SubscriptionData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
