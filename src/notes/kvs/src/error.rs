//! 这里使用的是 failure crate，具体的使用指南可以参考 https://boats.gitlab.io/failure/

use std::io;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "key not found")]
    KeyNotFound,

    #[fail(display = "serde json error")]
    ToJsonErr(#[cause] serde_json::Error),

    #[fail(display = "proto error")]
    ProtoErr(#[cause] protobuf::ProtobufError),

    /// 不支持的指令操作类型
    #[fail(display = "Not support command type")]
    UnsupportCmdType,

    #[fail(display = "error: {}", reason)]
    CommonErr { reason: String },
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::ToJsonErr(err)
    }
}

impl From<protobuf::ProtobufError> for KvsError {
    fn from(err: protobuf::ProtobufError) -> Self {
        KvsError::ProtoErr(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
