#![allow(dead_code)]
use anyhow::{Error, Result};
use cxx::SharedPtr;

use std::marker::PhantomData;

// ---------------------------
// type state pattern
// ---------------------------

pub struct Device<Type = UnknownDeviceType, OS = UnknownOS>
where
    Type: DeviceType,
    OS: DeviceOS,
{
    _num: usize,
    dtype: Type,
    _os: PhantomData<OS>,
}

impl Device {
    pub fn fetch_device(num: usize) -> Result<Device<AnyDevice, AnyOS>> {
        if num < 1 {
            Err(Error::msg("num < 1 not allowed"))
        } else {
            let device = intern::ffi::fetch_device(num - 1)?;
            if device.is_null() {
                Err(Error::msg(format!("device {num} not found")))
            } else {
                Ok(Device {
                    _num: num,
                    dtype: AnyDevice(device),
                    _os: PhantomData,
                })
            }
        }
    }

    pub fn fetch_hsm(num: usize) -> Result<Device<Hsm, AnyOS>> {
        if num < 1 {
            Err(Error::msg("num < 1 not allowed"))
        } else {
            let device = intern::ffi::fetch_hsm(num - 1)?;
            if device.is_null() {
                Err(Error::msg(format!("HSM device {num} not found")))
            } else {
                Ok(Device {
                    _num: num,
                    dtype: Hsm(device),
                    _os: PhantomData,
                })
            }
        }
    }
}

/// make methods like os() and dtype() available for Device
impl<OS> std::ops::Deref for Device<AnyDevice, OS>
where
    OS: AnyDeviceOS,
{
    type Target = intern::ffi::Device;
    fn deref(&self) -> &Self::Target {
        &self.dtype.0
    }
}

/// make methods like os() and dtype() available for Hsm
impl<OS> std::ops::Deref for Device<Hsm, OS>
where
    OS: AnyDeviceOS,
{
    type Target = intern::ffi::HSM;
    fn deref(&self) -> &Self::Target {
        &self.dtype.0
    }
}

// states

pub trait DeviceOS {}
pub trait AnyDeviceOS: DeviceOS {}
pub struct UnknownOS;
pub struct AnyOS;
pub struct BareMetal;
pub struct Linux;
pub struct WinDoof;
impl DeviceOS for UnknownOS {}
impl DeviceOS for AnyOS {}
impl AnyDeviceOS for AnyOS {}
impl DeviceOS for BareMetal {}
impl AnyDeviceOS for BareMetal {}
impl DeviceOS for Linux {}
impl AnyDeviceOS for Linux {}
impl DeviceOS for WinDoof {}
impl AnyDeviceOS for WinDoof {}

pub trait DeviceType {}
pub trait AnyDeviceType: DeviceType {}
pub struct UnknownDeviceType;
pub struct AnyDevice(SharedPtr<intern::ffi::Device>);
pub struct Hsm(SharedPtr<intern::ffi::HSM>);
pub struct Fido;
impl DeviceType for UnknownDeviceType {}
impl DeviceType for AnyDevice {}
impl AnyDeviceType for AnyDevice {}
impl DeviceType for Hsm {}
impl AnyDeviceType for Hsm {}
impl DeviceType for Fido {}
impl AnyDeviceType for Fido {}

pub(super) mod intern {
    #[cxx::bridge(namespace = "cxx::device")]
    pub mod ffi {

        #[derive(Debug)]
        pub enum DeviceOS {
            BareMetal,
            Linux,
            WinDoof,
        }

        #[derive(Debug)]
        pub enum DeviceType {
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

            // HSM
            fn os(self: &HSM) -> DeviceOS;
            #[cxx_name = "type"]
            fn dtype(self: &HSM) -> DeviceType;

            //fn sign(self: &USB_HSM);
        }
    }
}
