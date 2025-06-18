use thiserror::Error;

use std::result;

use crate::cxx::intern::ffi::DeviceOS;

pub type Result<T> = result::Result<T, Error>;

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
    #[error(transparent)]
    CXXException(#[from] cxx::Exception),
}
