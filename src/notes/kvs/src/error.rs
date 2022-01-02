//! 这里使用的是 failure crate，具体的使用指南可以参考 https://boats.gitlab.io/failure/

use std::io;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "key not found")]
    KeyNotFound,

    #[fail(display = "error: {}", reason)]
    CommonErr {
        reason: String
    }
}


