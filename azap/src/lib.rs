pub mod prelude;
pub use axum::{
    self,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Json, Response},
    serve, Router,
};

/// Route Metadata storing
#[derive(Debug, Clone, Copy)]
pub struct RouteMetaData {
    pub method: &'static str,
    pub path: &'static str,
    pub handler_name: &'static str,
    pub module: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

pub use azap_macros::{delete, get, guards, patch, post, put, register_guard};
