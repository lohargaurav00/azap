// routes/auth/login.rs

use azap::post;

#[post("/login")]
pub async fn login() -> &'static str {
    // Simple and direct
    "login"
}
