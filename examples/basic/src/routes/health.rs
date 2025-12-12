// routes/health.rs

use azap::{get, put};

#[get("/")]
pub fn get_health() -> &'static str {
    "get_health"
}

#[put("/health_route-with_put")]
pub fn put_route_health() -> &'static str {
    "put_route_health"
}
