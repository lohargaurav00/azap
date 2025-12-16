use azap::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
}

#[post("/")]
pub async fn create_user(Json(payload): Json<CreateUser>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "id": 123,
        "name": payload.name,
        "email": payload.email
    }))
}
