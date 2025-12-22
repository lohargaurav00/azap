// routes/health.rs
use azap::{get, guards};

#[get("/")]
#[guards(auth, rate_limit(100, 5))]
pub async fn health_check() -> &'static str {
    "OK"
}
