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
