#include "device.h"

#include <vector>

namespace device {

  std::vector<std::shared_ptr<Device>> list_of_devices() {
    return {
      std::make_shared<USB_HSM>(DeviceOS::BareMetal),
      std::make_shared<SERVER_HSM>(DeviceOS::Linux),
      std::make_shared<SERVER_HSM>(DeviceOS::WinDoof),
      std::make_shared<FIDO_TWO>(DeviceOS::Linux),
      std::make_shared<FIDO_ONE>(DeviceOS::WinDoof),
      std::make_shared<FIDO_ONE>(DeviceOS::BareMetal),
    };
  }

  Device::Device(DeviceOS os) : m_os(os) {
  }

  HSM::HSM(DeviceOS os) : Device(os) {
  }

  USB_HSM::USB_HSM(DeviceOS os) : HSM(os) {
  }

  SERVER_HSM::SERVER_HSM(DeviceOS os) : HSM(os) {
  }

  FIDO::FIDO(DeviceOS os) : Device(os) {
  }

  FIDO_ONE::FIDO_ONE(DeviceOS os) : FIDO(os) {
  }

  FIDO_TWO::FIDO_TWO(DeviceOS os) : FIDO(os) {
  }

  std::vector<uint8_t> USB_HSM::sign() const {
    return std::vector<uint8_t>({
        1, 2, 3, 4, 5, 6, 7, 8
    });
  }

  std::vector<uint8_t> SERVER_HSM::sign() const {
    return std::vector<uint8_t>({
        1,  2,  3,  4,  5,  6,  7,  8,
        9, 10, 11, 12, 13, 14, 15, 16
    });
  }

} // namespace device
