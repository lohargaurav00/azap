mod routes;

// Include the auto-generated router code
include!(concat!(env!("OUT_DIR"), "/generated_routes.rs"));

#[tokio::main]
async fn main() {
    // Initialize logging

    // Use the generated register_routes() function
    let app = register_routes();

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("⚡ Azap server running on http://127.0.0.1:3000");
    println!("📁 Routes auto-discovered from src/routes/");
    println!();
    println!("Try these endpoints:");
    println!("  GET  http://127.0.0.1:3000/health");
    println!("  GET  http://127.0.0.1:3000/users");
    println!("  GET  http://127.0.0.1:3000/users/123");
    println!("  POST http://127.0.0.1:3000/users");

    axum::serve(listener, app).await.unwrap();
}

