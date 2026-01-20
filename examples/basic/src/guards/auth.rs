use azap::{
    axum::{extract::Request, middleware::Next},
    register_guard, Response, State, StatusCode,
};

use crate::AppState;

#[register_guard(guard_type = "fn_with_state")]
pub async fn auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("Name : {}", state.name);
    Ok(next.run(req).await)
}
