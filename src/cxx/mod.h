#pragma once

#include "rust/cxx.h"
#include "device.h"

namespace cxx::device {

  using DeviceOS = ::device::DeviceOS;
  using DeviceType = ::device::DeviceType;

  using Device = ::device::Device;
  using HSM = ::device::HSM;

  auto fetch_device(size_t num) -> std::shared_ptr<Device>;
  auto fetch_hsm(size_t num) -> std::shared_ptr<HSM>;
}
