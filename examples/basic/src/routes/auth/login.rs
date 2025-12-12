// routes/auth/login.rs

use azap::post;

#[post("/")]
async fn login() -> &'static str {
    // Simple and direct
    "login"
}
