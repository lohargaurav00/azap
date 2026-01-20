// routes/health.rs
use azap::{get, guards};

#[get("/")]
#[guards(auth, tracing)]
pub async fn health_check() -> &'static str {
    "OK"
}
