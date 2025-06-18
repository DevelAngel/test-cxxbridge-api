use thiserror::Error;

use std::result;

use crate::cxx::intern::ffi::DeviceOS;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid device number {num}")]
    InvalidDeviceNum { num: usize },
    #[error("{device_type} device {num} not found")]
    DeviceNotFound {
        device_type: &'static str,
        num: usize,
    },
    #[error("{device_type} device {num} has wrong OS: {actual_os} instead of {expected_os}")]
    DeviceHasWrongOS {
        device_type: &'static str,
        num: usize,
        actual_os: DeviceOS,
        expected_os: DeviceOS,
    },
    #[error("CXX runtime error: {msg}")]
    CXXRuntime { msg: String },
    #[error("CXX exception: {msg}")]
    CXXException { msg: String },
}

pub type Result<T> = result::Result<T, Error>;

/// retrieve error classes from CXXException.msg()
impl From<cxx::Exception> for Error {
    fn from(src: cxx::Exception) -> Self {
        use lazy_regex::regex;
        let re = regex!(r"\[(?<cat>\w+)\]\s*(?<msg>.*)");
        let (_, [cat, msg]) = re.captures(src.what()).unwrap().extract();
        let msg = msg.to_owned();
        match cat {
            "RUNTIME" => Error::CXXRuntime { msg },
            "STD" => Error::CXXException { msg },
            "UNKNOWN" => Error::CXXException {
                msg: "unknown error".to_owned(),
            },
            _ => Error::CXXException {
                msg: format!("unhandled error: {msg}"),
            },
        }
    }
}
