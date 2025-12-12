// routes/users/get.rs
use azap::get;

#[get("/")]
async fn list_users() -> &'static str {
    // Implementation
    "list_users"
}

#[get("/:id")]
async fn get_by_id() -> &'static str {
    // Implementation
    "get_by_id"
}
