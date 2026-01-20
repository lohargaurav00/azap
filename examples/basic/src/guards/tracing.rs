use azap::{
    axum::{extract::Request, middleware::Next},
    register_guard, Response, StatusCode,
};

#[register_guard(guard_type = "fn")]
pub async fn tracing(req: Request, next: Next) -> Result<Response, StatusCode> {
    dbg!(&req);
    Ok(next.run(req).await)
}
