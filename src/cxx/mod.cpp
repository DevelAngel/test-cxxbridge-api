#include "test-cxxbridge-api/src/cxx/mod.h"
#include "device.h"

namespace cxx::device {

  auto fetch_device(size_t num) -> std::shared_ptr<Device> {
    auto list = ::device::list_of_devices();
    auto device = list.at(num);
    return device;
  }

  auto fetch_hsm(size_t num) -> std::shared_ptr<HSM> {
    auto list = ::device::list_of_devices();
    auto device = list.at(num);
    return std::dynamic_pointer_cast<HSM>(device);
  }

} // namespace
