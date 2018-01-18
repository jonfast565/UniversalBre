#pragma once

#include "global_defines.h"

namespace core {
    // initialize whitespace definition
    const wchar_t whitespace_chars[] = { ' ', '\r\n', '\n', '\v', '\t' };
    const int whitespace_chars_length = 5;

    // initialize newline whitespace chars
    const wchar_t newline_chars[] = { '\r\n', '\n' };
    const int newline_chars_length = 2;

    // initialize breaking characters definition
    const wchar_t breaking_chars[] = { '(', ')', '+', '-', '*', '/', '~', '.' };
    const int breaking_chars_length = 8;

    // initialize breaking characters specifically for numeric scanning
    const wchar_t integer_breaking_chars[] = { '(', ')', '+', '-', '*', '/', '~' };
    const int integer_breaking_chars_length = 7;
}

class utility {
public:
    // scanner ops
    static bool is_digit(const wchar_t possible_digit);
    static bool is_alpha(const wchar_t possible_alpha);
    static bool is_alpha_digit(const wchar_t possible_alpha_digit);
    static bool is_dot(const wchar_t possible_dot);
    static bool is_underscore(const wchar_t possible_underscore);
    static bool is_specific_char(const wchar_t current_char, const wchar_t possible_specific_char, bool case_invariant);
    static bool is_charset(const wchar_t possible_charset, const std::vector<wchar_t> charset);
    static bool is_not_charset(const wchar_t possible_charset, const std::vector<wchar_t> charset);
    static bool is_whitespace(const wchar_t possible_whitespace);
    static bool is_not_whitespace(const wchar_t possible_whitespace);
    static bool is_break_char(const wchar_t possible_break_char);
    static bool is_integer_break_char(const wchar_t possible_break_char);
    static bool is_newline_char(const wchar_t possible_newline_char);

    // array utils
    template <typename T>
    static std::vector<T> array_to_vector(const T arr[], const int arr_length);

    // string utils
    static const char* wstring_to_cstr(std::wstring& wide_string);
    static const wchar_t* wstring_to_wcstr(std::wstring& wide_string);
    static std::wstring cstr_to_wstring(const char* c_str);
    static std::wstring build_indent_str(int indent);

    // string concat utils
    static const void concat_in_place(const wchar_t ** result, const int count, ...);
    static const void concat_two(const wchar_t ** result, const wchar_t * one, const wchar_t * two);
    static const void concat_three(const wchar_t ** result, const wchar_t * one, const wchar_t * two, const wchar_t * three);

    // pointer utils
    template <typename T>
    static std::shared_ptr<T> make_ptr_s(const T obj);

    template <typename T>
    static std::unique_ptr<T> make_ptr_u(const T obj);
};

template<typename T>
inline std::vector<T> utility::array_to_vector(const T arr[], const int arr_length)
{
    auto new_vec = std::vector<T>();
    for (int i = 0; i < arr_length; i++) {
        new_vec.push_back(arr[i]);
    }
    return new_vec;
}

template<typename T>
inline std::shared_ptr<T> utility::make_ptr_s(T obj)
{
    return std::make_shared<T>(obj);
}

template<typename T>
inline std::unique_ptr<T> utility::make_ptr_u(T obj)
{
    return std::move(std::make_unique<T>(obj));
}

// trim from start (in place)
static inline void ltrim(std::wstring &s) {
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](int ch) {
        return !std::isspace(ch);
    }));
}

// trim from end (in place)
static inline void rtrim(std::wstring &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(), [](int ch) {
        return !std::isspace(ch);
    }).base(), s.end());
}

// trim from both ends (in place)
static inline void trim(std::wstring &s) {
    ltrim(s);
    rtrim(s);
}

// trim from start (copying)
static inline std::wstring ltrim_copy(std::wstring s) {
    ltrim(s);
    return s;
}

// trim from end (copying)
static inline std::wstring rtrim_copy(std::wstring s) {
    rtrim(s);
    return s;
}

// trim from both ends (copying)
static inline std::wstring trim_copy(std::wstring s) {
    trim(s);
    return s;
}
