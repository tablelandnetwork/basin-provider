use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn health_check() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}
