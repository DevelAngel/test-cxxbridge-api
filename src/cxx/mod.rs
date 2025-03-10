use anyhow::{Error, Result};
use cxx::SharedPtr;

pub struct Device(SharedPtr<intern::ffi::Device>);
pub struct Hsm(SharedPtr<intern::ffi::HSM>);

impl std::ops::Deref for Device {
    type Target = intern::ffi::Device;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for Hsm {
    type Target = intern::ffi::HSM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn fetch_device(num: usize) -> Result<Device> {
    if num < 1 {
        Err(Error::msg("num < 1 not allowed"))
    } else {
        let device = intern::ffi::fetch_device(num - 1)?;
        if device.is_null() {
            Err(Error::msg(format!("device {num} not found")))
        } else {
            Ok(Device(device))
        }
    }
}

pub fn fetch_hsm(num: usize) -> Result<Hsm> {
    if num < 1 {
        Err(Error::msg("num < 1 not allowed"))
    } else {
        let device = intern::ffi::fetch_hsm(num - 1)?;
        if device.is_null() {
            Err(Error::msg(format!("HSM device {num} not found")))
        } else {
            Ok(Hsm(device))
        }
    }
}

pub(super) mod intern {
    #[cxx::bridge(namespace = "cxx::device")]
    pub mod ffi {

        #[derive(Debug)]
        enum DeviceOS {
            BareMetal,
            Linux,
            WinDoof,
        }

        #[derive(Debug)]
        enum DeviceType {
            HSM,
            FIDO,
        }

        unsafe extern "C++" {
            include!("test-cxxbridge-api/src/cxx/mod.h");

            type DeviceOS;
            type DeviceType;
            type Device;
            type HSM;

            fn fetch_device(num: usize) -> Result<SharedPtr<Device>>;
            fn fetch_hsm(num: usize) -> Result<SharedPtr<HSM>>;

            // Device
            fn os(self: &Device) -> DeviceOS;
            #[cxx_name = "type"]
            fn dtype(self: &Device) -> DeviceType;

            // Device
            fn os(self: &HSM) -> DeviceOS;
            #[cxx_name = "type"]
            fn dtype(self: &HSM) -> DeviceType;

            //fn sign(self: &USB_HSM);
        }
    }
}
