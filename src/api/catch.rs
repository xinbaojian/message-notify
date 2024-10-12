use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

/// 拦截服务器内部错误
#[catch(500)]
pub fn internal_error() -> String {
    "Whoops! There was an internal error. Try again later.".to_string()
}
