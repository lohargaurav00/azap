// routes/health.rs
use azap::{get, guards};

#[get("/")]
#[guards(auth)]
pub async fn health_check() -> &'static str {
    "OK"
}
