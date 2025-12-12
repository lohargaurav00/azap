// routes/users/create.rs
use azap::post;

#[post("/")]
async fn create_user() -> &'static str {
    // Implementation
    "create_user"
}

#[post("/new-user")]
async fn create_user_new() -> &'static str {
    // Implementation
    "create_user_new_user"
}
