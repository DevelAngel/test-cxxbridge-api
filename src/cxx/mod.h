#pragma once

#include "rust/cxx.h"
#include "extern/device.h"

namespace cxx::device {

  using DeviceOS = ::device::DeviceOS;
  using DeviceType = ::device::DeviceType;

  using Device = ::device::Device;
  using HSM = ::device::HSM;
  using USB_HSM = ::device::USB_HSM;
  using SERVER_HSM = ::device::SERVER_HSM;

  struct HSMWrapper;

  auto fetch_device(rust::usize num) -> std::shared_ptr<Device>;
  auto fetch_hsm(rust::usize num) -> std::shared_ptr<HSM>;

}
