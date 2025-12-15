// routes/auth/register.rs

use azap::post;

#[post("/register")]
pub async fn register() -> &'static str {
    "register"
}
