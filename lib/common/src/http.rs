use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

/// Error message used by HTTP API
#[derive(Serialize)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

// HTTP API rejection handler
pub async fn handle_warp_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };
    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });
    Ok(warp::reply::with_status(json, code))
}
