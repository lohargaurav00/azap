// routes/health.rs

use azap::{get, put};

#[get("/")]
pub async fn get_health() -> &'static str {
    "get_health"
}

#[put("/health_route-with_put")]
pub async fn put_route_health() -> &'static str {
    "put_route_health"
}
