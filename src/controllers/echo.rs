use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn get_root() -> Json<Value> {
    let transaction = Uuid::new_v4();
    let host = hostname::get()
        .unwrap_or_else(|_| "unknown".into())
        .into_string()
        .unwrap_or_else(|_| "unknown".into());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".into());
    let current_time = chrono::Utc::now();
    Json(json!({
        "code": "echo",
        "transaction": transaction.to_string(),
        "message": "OK",
        "args": [],
        "data": {
            "server": host,
            "time": current_time.to_rfc3339(),
            "version": version,
        }
    }))
}
