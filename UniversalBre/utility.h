#pragma once

#include <vector>
#include <locale>
#include <codecvt>
#include <algorithm>
#include <string>

#include <stdio.h>
#include <wchar.h>
#include <wctype.h>

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

    // array utils
    template <typename T>
    static std::vector<T> array_to_vector(const T arr[]);

    // string utils
    static const char* wstring_to_cstr(std::wstring wide_string);
    static std::wstring cstr_to_wstring(const char* c_str);
};

namespace core {
    // initialize whitespace definition
    const wchar_t whitespace_chars[] = { ' ', '\r\n', '\n', '\v', '\t' };
}

template<typename T>
inline std::vector<T> utility::array_to_vector(const T arr[])
{
    return std::vector<T>(arr, arr + sizeof arr / sizeof T);
}
