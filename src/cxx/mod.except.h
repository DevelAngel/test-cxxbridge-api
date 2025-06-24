#pragma once

namespace rust::behavior {

  /* NOTE: Each error class must be declared here.
   *       Otherwise, it would not be called.
   */
  extern auto retrieve_error(std::runtime_error e) noexcept -> std::string;
  extern auto retrieve_error(std::exception e) noexcept -> std::string;
  extern auto retrieve_unknown_error(void) noexcept -> std::string;

  template <typename Try, typename Fail>
  static void trycatch(Try &&func, Fail &&fail) noexcept try {
    func();
  } catch (std::runtime_error const& e) {
    fail(retrieve_error(e));
  } catch (std::exception const& e) {
    fail(retrieve_error(e));
  } catch (...) {
    fail(retrieve_unknown_error());
  }

}
