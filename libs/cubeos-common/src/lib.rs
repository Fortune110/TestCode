use thiserror::Error;
use serde::{Serialize, Deserialize};

/// 公共库示例作为测试
pub fn hello_common() {
    println!("Hello from cubeos-common!");
}

/// 测试错误
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum CommonError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unknown error")]
    Unknown,
}

pub type CommonResult<T> = Result<T, CommonError>;