use crate::ctx::Ctx;
use crate::errors::ClientError;
use crate::Error;
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::time::SystemTime;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Serialize)]
#[skip_serializing_none] // None doesn't get serialized
struct RequestLogLine {
    uuid: String,
    timestamp: String, // should be ISO 8901

    user_id: Option<u64>,

    req_path: String,
    req_method: String,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_err: Option<&Error>,
    client_err: Option<ClientError>,
) {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_err.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_err)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|d| d.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: time.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_err.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    println!(">> LOG REQUEST: \n{}", json!(log_line));

    // TODO: Send to CloudWatch
}
