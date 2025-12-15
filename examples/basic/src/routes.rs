use azap::get;

pub mod auth;
pub mod health;
pub mod users;

#[get("/")]
pub async fn root_fn() -> &'static str {
    "root_fn"
}
