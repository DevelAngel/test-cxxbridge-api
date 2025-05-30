#include "test-cxxbridge-api/src/cxx/mod.rs.h" // generated by cxx.rs
#include "rust/cxx.h" // provided by cxx.rs
#include "device.h"

namespace cxx::device {

  /// std::vector<u8> -> rust::Vec<u8>
  static auto convert_vec(std::vector<uint8_t> const& data_in) -> rust::Vec<rust::u8> {
    rust::Vec<rust::u8> data_out;
    data_out.reserve(data_in.size());
    std::copy(data_in.cbegin(), data_in.cend(), std::back_inserter(data_out));
    return data_out;
  }

  auto fetch_device(rust::usize num) -> std::shared_ptr<Device> {
    auto list = ::device::list_of_devices();
    auto device = list.at(num);
    return device;
  }

  auto fetch_hsm(rust::usize num) -> std::shared_ptr<HSM> {
    auto list = ::device::list_of_devices();
    auto device = list.at(num);
    return std::dynamic_pointer_cast<HSM>(device);
  }

  // HSMWrapper

  auto HSMWrapper::sign(rust::usize slot) const -> rust::Vec<rust::u8> {
    assert(this->intern);
    auto sig = this->intern->sign(slot);
    return convert_vec(sig);
  }

} // namespace
