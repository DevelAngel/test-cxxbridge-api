#pragma once

#include "rust/cxx.h"
#include "extern/device.h"

namespace cxx::device {

  using DeviceOS = ::device::DeviceOS;
  using DeviceType = ::device::DeviceType;

  using Device = ::device::Device;
  using HSM = ::device::HSM;

  struct HSMWrapper;

  auto fetch_device(rust::usize num) -> std::shared_ptr<Device>;
  auto fetch_hsm(rust::usize num) -> std::shared_ptr<HSM>;

}
