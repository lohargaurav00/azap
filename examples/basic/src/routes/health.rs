// routes/health.rs
use azap::{get, guards};

#[guards(auth)]
#[get("/")]
pub async fn health_check() -> &'static str {
    "OK"
}
