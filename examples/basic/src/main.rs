use azap::prelude::*;

#[get("/health")]
async fn health() -> &'static str {
    "OK"
}

#[post("/user")]
async fn users() -> &'static str {
    "user"
}

#[tokio::main]
async fn main() {
    // Simple health check
    let _ = health().await;
    println!("Hello, world!");
}
