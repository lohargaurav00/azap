use azap::prelude::*;

#[get("/")]
pub async fn list_users() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"},
        ]
    }))
}

#[get("/{id}")]
pub async fn get_user(Path(id): Path<u32>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "id": id,
        "name": "User Name"
    }))
}
