#![allow(dead_code)]
use crate::error::{Error, Result};

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
            Err(Error::InvalidDeviceNum { num })
        } else {
            let device = intern::ffi::fetch_device(num - 1)?;
            if device.is_null() {
                Err(Error::DeviceNotFound {
                    device_type: "any",
                    num,
                })
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
            Err(Error::InvalidDeviceNum { num })
        } else {
            let device = intern::ffi::fetch_hsm(num - 1)?;
            if device.is_null() {
                Err(Error::DeviceNotFound {
                    device_type: "HSM",
                    num,
                })
            } else {
                let device = intern::ffi::HSMWrapper { intern: device };
                Ok(Device {
                    _num: num,
                    dtype: Hsm(device),
                    _os: PhantomData,
                })
            }
        }
    }

    pub fn fetch_hsm_with<OS: AnyDeviceOS>(num: usize) -> Result<Device<Hsm, OS>> {
        if num < 1 {
            Err(Error::InvalidDeviceNum { num })
        } else {
            let device = intern::ffi::fetch_hsm(num - 1)?;
            if device.is_null() {
                Err(Error::DeviceNotFound {
                    device_type: "HSM",
                    num,
                })
            } else if device.os() != OS::OS {
                Err(Error::DeviceHasWrongOS {
                    device_type: "HSM",
                    num,
                    actual_os: device.os(),
                    expected_os: OS::OS,
                })
            } else {
                let device = intern::ffi::HSMWrapper { intern: device };
                Ok(Device {
                    _num: num,
                    dtype: Hsm(device),
                    _os: PhantomData,
                })
            }
        }
    }
}

impl<OS> Device<Hsm, OS>
where
    OS: DeviceOS,
{
    pub fn sign_slot(&self, slot: usize) -> Result<Vec<u8>> {
        let sig = self.dtype.0.sign(slot)?;
        Ok(sig)
    }
}

impl<Type> Device<Type, Linux>
where
    Type: AnyDeviceType,
{
    pub fn name(&self) -> String {
        let device = self.dtype.device();
        let name = device.name();
        let name = name.to_str().expect("valid name");
        name.to_owned()
    }
}

// Deref

/// make methods like os() and dtype() available for Device
impl<OS> std::ops::Deref for Device<AnyDevice, OS>
where
    OS: DeviceOS,
{
    type Target = intern::ffi::Device;
    fn deref(&self) -> &Self::Target {
        &self.dtype.0
    }
}

/// make methods like os() and dtype() available for Hsm
impl<OS> std::ops::Deref for Device<Hsm, OS>
where
    OS: DeviceOS,
{
    type Target = intern::ffi::HSMWrapper;
    fn deref(&self) -> &Self::Target {
        &self.dtype.0
    }
}

/// make methods like create_key() available for Hsm
impl<OS> std::ops::DerefMut for Device<Hsm, OS>
where
    OS: DeviceOS,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dtype.0
    }
}

/// make methods like os() and dtype() available for HSMWrapper
impl std::ops::Deref for intern::ffi::HSMWrapper {
    type Target = intern::ffi::HSM;
    fn deref(&self) -> &Self::Target {
        &self.intern
    }
}

// states

pub trait DeviceOS {}
pub trait AnyDeviceOS: DeviceOS {
    const OS: intern::ffi::DeviceOS;
}
pub struct UnknownOS;
pub struct AnyOS;
pub struct BareMetal;
pub struct Linux;
pub struct WinDoof;
impl DeviceOS for UnknownOS {}
impl DeviceOS for AnyOS {}
impl DeviceOS for BareMetal {}
impl AnyDeviceOS for BareMetal {
    const OS: intern::ffi::DeviceOS = intern::ffi::DeviceOS::BareMetal;
}
impl DeviceOS for Linux {}
impl AnyDeviceOS for Linux {
    const OS: intern::ffi::DeviceOS = intern::ffi::DeviceOS::Linux;
}
impl DeviceOS for WinDoof {}
impl AnyDeviceOS for WinDoof {
    const OS: intern::ffi::DeviceOS = intern::ffi::DeviceOS::WinDoof;
}

pub trait DeviceType {}
pub trait AnyDeviceType: DeviceType {
    fn device(&self) -> SharedPtr<intern::ffi::Device>;
}
pub struct UnknownDeviceType;
pub struct AnyDevice(SharedPtr<intern::ffi::Device>);
pub struct Hsm(intern::ffi::HSMWrapper);
pub struct Fido;
impl DeviceType for UnknownDeviceType {}
impl DeviceType for AnyDevice {}
impl AnyDeviceType for AnyDevice {
    fn device(&self) -> SharedPtr<intern::ffi::Device> {
        self.0.clone()
    }
}
impl DeviceType for Hsm {}
impl AnyDeviceType for Hsm {
    fn device(&self) -> SharedPtr<intern::ffi::Device> {
        self.0.device()
    }
}
impl DeviceType for Fido {}
impl AnyDeviceType for Fido {
    fn device(&self) -> SharedPtr<intern::ffi::Device> {
        SharedPtr::null()
    }
}

pub(super) mod intern {
    use std::fmt;

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

        pub struct HSMWrapper {
            intern: SharedPtr<HSM>,
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
            fn name(self: &Device) -> &CxxString;

            // HSM
            fn os(self: &HSM) -> DeviceOS;
            #[cxx_name = "type"]
            fn dtype(self: &HSM) -> DeviceType;
            fn max_slots(self: &HSM) -> usize;
            //fn sign(self: &HSM, slot: usize) -> CxxVector<u8>; //< error: returning C++ vector by value is not supported

            // HSMWrapper
            fn device(self: &HSMWrapper) -> SharedPtr<Device>;
            fn sign(self: &HSMWrapper, slot: usize) -> Result<Vec<u8>>;
            fn create_key(self: &mut HSMWrapper, slot: usize) -> Result<()>;
        }
    }

    impl fmt::Display for ffi::DeviceType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ffi::DeviceType::HSM => write!(f, "HSM"),
                ffi::DeviceType::FIDO => write!(f, "FIDO"),
                ffi::DeviceType {
                    repr: 2_u8..=u8::MAX,
                } => unreachable!("invalid Device Type"),
            }
        }
    }

    impl fmt::Display for ffi::DeviceOS {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ffi::DeviceOS::BareMetal => write!(f, "BareMetal"),
                ffi::DeviceOS::Linux => write!(f, "Linux"),
                ffi::DeviceOS::WinDoof => write!(f, "WinDoof"),
                ffi::DeviceOS {
                    repr: 3_u8..=u8::MAX,
                } => unreachable!("invalid Device OS"),
            }
        }
    }
}
