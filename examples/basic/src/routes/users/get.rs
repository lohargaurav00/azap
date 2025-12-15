// routes/users/get.rs
use azap::get;

#[get("/")]
pub async fn list_users() -> &'static str {
    // Implementation
    "list_users"
}

#[get("/:id")]
pub async fn get_by_id() -> &'static str {
    // Implementation
    "get_by_id"
}
