#pragma once 

#include <memory>
#include <vector>
#include <string>
#include <cstdint>

namespace device {

  enum class DeviceType: ::std::uint8_t {
    HSM,
    FIDO,
  };

  enum class DeviceOS: ::std::uint8_t {
    BareMetal,
    Linux,
    WinDoof,
  };

  /** Common properties of all devices.
   *
   * Each device has an Operating System (OS).
   * Furthermore, the devices can be distinuished from each other by their type.
   */
  class Device {
    public:
      Device() = delete;
      explicit Device(DeviceOS os, std::string name);
      virtual ~Device() {}

      virtual DeviceType type() const noexcept = 0;
      virtual DeviceOS os() const noexcept final { return m_os; }
      virtual std::string const& name() const noexcept final { return m_name; }

    private:
      DeviceOS m_os;
      std::string m_name; // only used by Linux
  };

  /**
   * Devices of type A (HSM).
   */
  class HSM: public Device {
    public:
      HSM() = delete;
      explicit HSM(DeviceOS os, std::string name);
      virtual ~HSM() {}

      virtual DeviceType type() const noexcept final { return DeviceType::HSM; }

      /**
       * Maximum number of slots.
       */
      virtual size_t max_slots() const noexcept = 0;

      /**
       * Each HSM provide a sign method.
       *
       * Typically for C++ implementations, the signature is returned by value.
       */
      virtual std::vector<uint8_t> sign(size_t slot) const = 0;
  };

  /// variant one of devices of type A
  class USB_HSM: public HSM {
    public:
      USB_HSM() = delete;
      explicit USB_HSM(DeviceOS os);
      explicit USB_HSM(DeviceOS os, std::string name);
      virtual ~USB_HSM() {}

      virtual size_t max_slots() const noexcept final { return 2; }
      virtual std::vector<uint8_t> sign(size_t slot) const final;
      virtual void generate_rsa_key(size_t slot) final;

    private:
      bool m_rsa_key_generated_slot_one = false;
      bool m_rsa_key_generated_slot_two = false;
  };

  /// variant two of devices of type A
  class SERVER_HSM: public HSM {
    public:
      SERVER_HSM() = delete;
      explicit SERVER_HSM(DeviceOS os);
      explicit SERVER_HSM(DeviceOS os, std::string name);
      virtual ~SERVER_HSM() {}

      virtual size_t max_slots() const noexcept final { return 5; }
      virtual std::vector<uint8_t> sign(size_t slot) const final;
      virtual void generate_secp256k1_key(size_t slot) final;

    private:
      bool m_secp256k1_key_generated_slot_one = false;
      bool m_secp256k1_key_generated_slot_two = false;
      bool m_secp256k1_key_generated_slot_three = false;
      bool m_secp256k1_key_generated_slot_four = false;
      bool m_secp256k1_key_generated_slot_five = false;
  };

  /// devices of type B
  class FIDO: public Device {
    public:
      FIDO() = delete;
      explicit FIDO(DeviceOS os, std::string name);
      virtual ~FIDO() {}

      virtual DeviceType type() const noexcept final { return DeviceType::FIDO; }

    private:
      DeviceOS m_os;
  };

  /// variant one of devices of type B
  class FIDO_ONE: public FIDO {
    public:
      FIDO_ONE() = delete;
      explicit FIDO_ONE(DeviceOS os);
      explicit FIDO_ONE(DeviceOS os, std::string name);
      virtual ~FIDO_ONE() {}
  };

  /// variant two of devices of type B
  class FIDO_TWO: public FIDO {
    public:
      FIDO_TWO() = delete;
      explicit FIDO_TWO(DeviceOS os);
      explicit FIDO_TWO(DeviceOS os, std::string name);
      virtual ~FIDO_TWO() {}
  };

  std::vector<std::shared_ptr<Device>> list_of_devices();
}
