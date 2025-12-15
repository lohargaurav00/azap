// routes/users/create.rs
use azap::post;

#[post("/")]
pub async fn create_user() -> &'static str {
    // Implementation
    "create_user"
}

#[post("/new-user")]
pub async fn create_user_new() -> &'static str {
    // Implementation
    "create_user_new_user"
}
