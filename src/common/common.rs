use serde::{Deserialize, Serialize};

/// 通用返回结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Result<T> {
    pub data: T,
    pub code: ResultCode,
    pub success: bool,
}

/// 通用返回枚举
#[derive(Debug, Serialize, Deserialize)]
pub enum ResultCode {
    Success = 200,
    // 400 状态吗
    BadRequest = 400,
    // 服务器内部错误
    InternalError = 500,
}

impl<T> Result<T> {
    // 默认构造函数
    pub fn new(data: T, code: ResultCode, success: bool) -> Result<T> {
        Result {
            data,
            code,
            success,
        }
    }

    // 成功
    #[allow(dead_code)]
    pub fn success(data: T) -> Result<T> {
        Result::new(data, ResultCode::Success, true)
    }

    // 失败
    #[allow(dead_code)]
    pub fn fail_with_code(data: T, code: ResultCode) -> Result<T> {
        Result::new(data, code, false)
    }

    // BadRequest
    pub fn fail_bad_request(data: T) -> Result<T> {
        Result::new(data, ResultCode::BadRequest, false)
    }

    // 服务器内部错误
    pub fn fail_internal_error(data: T) -> Result<T> {
        Result::new(data, ResultCode::InternalError, false)
    }
}
