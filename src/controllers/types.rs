use axum::{body::Body, http::Response, response::IntoResponse};
use serde_json::{json, Value};
use uuid::Uuid;

pub struct APIError {
    pub code: String,
    pub message: String,
    pub args: Vec<String>,
    pub data: Value,
}

impl From<anyhow::Error> for APIError {
    fn from(value: anyhow::Error) -> Self {
        APIError {
            code: "error".into(),
            message: format!("{:?}", value),
            args: vec![],
            data: json!({}),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response<Body> {
        let transaction = Uuid::new_v4();
        let response = json!({
            "code": self.code,
            "transaction": transaction.to_string(),
            "message": self.message,
            "args": self.args,
            "data": self.data,
        });
        axum::http::Response::builder()
            .status(500)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&response).unwrap()))
            .unwrap()
    }
}
