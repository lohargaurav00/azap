// routes/auth/register.rs

use azap::post;

#[post("/")]
async fn register() -> &'static str {
    "register"
}
