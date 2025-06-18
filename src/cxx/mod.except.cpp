#include <stdexcept>
#include <exception>
#include <string>

namespace rust::behavior {

  using namespace std::string_literals;

  // customized error handling
  template <typename Try, typename Fail>
  static void trycatch(Try &&func, Fail &&fail) noexcept try {
    func();
  } catch (std::runtime_error const& e) {
    fail("[RUNTIME] "s + e.what());
  } catch (std::exception const& e) {
    fail("[STD] "s + e.what());
  } catch (...) {
    fail("[UNKNOWN]");
  }

}
