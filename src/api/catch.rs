use crate::common::Result;
use rocket::serde::json::Json;
use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> Json<Result<String>> {
    Json(Result::fail_bad_request(format!(
        "Resource {} not found",
        req.uri()
    )))
}

/// 拦截服务器内部错误
#[catch(500)]
pub fn internal_error() -> Json<Result<String>> {
    Json(Result::fail_internal_error(
        "Whoops! There was an internal error. Try again later.".to_string(),
    ))
}
