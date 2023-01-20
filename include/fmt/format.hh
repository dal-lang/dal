/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_FMT_FORMAT_HH
#define DAL_FMT_FORMAT_HH

#include <stdexcept>
#include <string>

namespace fmt {

// Inline std::is_same_v.
template<typename T, typename U>
inline constexpr bool is_same_v = std::is_same<T, U>::value;

template<typename T>
std::string _format(T value) {
  // FIXME: This is a temporary solution.
  if constexpr (is_same_v<T, int>) {
    return std::to_string(value);
  } else if constexpr (is_same_v<T, float>) {
    return std::to_string(value);
  } else if constexpr (is_same_v<T, double>) {
    return std::to_string(value);
  } else if constexpr (is_same_v<T, char>) {
    return std::string(1, value);
  } else if constexpr (is_same_v<T, std::string_view>) {
    return std::string(value);
  } else if constexpr (is_same_v<T, std::string>) {
    return value;
  } else if constexpr (is_same_v<T, bool>) {
    return value ? "true" : "false";
  } else {
    // if we reach here, it doesn't mean that the type is not supported.
    // it can be typed of const char* or const char[] which is same as std::string.
    // or it can be type pointer to the type above.
    // so we need to check if the type is convertible to the type above.
    if constexpr (std::is_convertible_v<T, int>) {
      return _format(static_cast<int>(value));
    } else if constexpr (std::is_convertible_v<T, float>) {
      return _format(static_cast<float>(value));
    } else if constexpr (std::is_convertible_v<T, double>) {
      return _format(static_cast<double>(value));
    } else if constexpr (std::is_convertible_v<T, char>) {
      return _format(static_cast<char>(value));
    } else if constexpr (std::is_convertible_v<T, std::string>) {
      return _format(static_cast<std::string>(value));
    } else if constexpr (std::is_convertible_v<T, bool>) {
      return _format(static_cast<bool>(value));
    } else {
      // if we reach here, it means that the type is not supported.
      throw std::runtime_error("Type is not supported.");
    }
  }
}

template<typename... Args>
std::string format(std::string format, Args... args) {
  std::string result;
  auto args_list = {_format(args)...};

  auto args_it = args_list.begin();
  for (auto format_it = format.begin(); format_it!=format.end();
       ++format_it) {
    if (*format_it=='{') {
      if (*(format_it + 1)=='}') {
        // to prevent bad alloc, we need to check if the args is empty.
        if (args_it==args_list.end()) {
          throw std::runtime_error("Too many placeholders.");
        }
        result += *args_it;
        ++args_it;
        ++format_it;
      } else {
        result += *format_it;
      }
    } else {
      result += *format_it;
    }
  }

  return result;
}

}  // namespace fmt

#endif //DAL_FMT_FORMAT_HH
