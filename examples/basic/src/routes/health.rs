// routes/health.rs
use azap::get;

#[get("/")]
pub async fn health_check() -> &'static str {
    "OK"
}
