use axum::{Router, response::Json, routing::get};
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/message", get(message))
}

async fn message() -> Json<serde_json::Value> {
    Json(json!({ "message": "All set! Time to build something awesome" }))
}
