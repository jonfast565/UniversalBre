#include "stdafx.h"
#include "utility.h"

bool utility::is_digit(const wchar_t possible_digit)
{
    return iswdigit(possible_digit) != 0;
}

bool utility::is_alpha(const wchar_t possible_alpha)
{
    return iswalpha(possible_alpha) != 0;
}

bool utility::is_alpha_digit(const wchar_t possible_alpha_digit)
{
    return is_digit(possible_alpha_digit) || is_alpha(possible_alpha_digit);
}

bool utility::is_dot(const wchar_t possible_dot)
{
    return possible_dot == L'.';
}

bool utility::is_underscore(const wchar_t possible_underscore)
{
    return possible_underscore == L'_';
}

bool utility::is_specific_char(const wchar_t current_char, const wchar_t possible_specific_char, bool case_invariant)
{
    if (!case_invariant) {
        return current_char == possible_specific_char;
    }
    return towlower(current_char) == towlower(possible_specific_char);
}

bool utility::is_charset(const wchar_t possible_charset, const std::vector<wchar_t> charset)
{
    for (auto char_iter : charset) {
        if (possible_charset == char_iter)
            return true;
    }
    return false;
}

bool utility::is_not_charset(const wchar_t possible_charset, const std::vector<wchar_t> charset)
{
    return !is_charset(possible_charset, charset);
}

bool utility::is_whitespace(const wchar_t possible_whitespace)
{
    return is_charset(possible_whitespace, array_to_vector<wchar_t>(core::whitespace_chars));
}

bool utility::is_not_whitespace(const wchar_t possible_whitespace)
{
    return !is_whitespace(possible_whitespace);
}

const char * utility::wstring_to_cstr(std::wstring wide_string)
{
    using convert_type = std::codecvt_utf8<wchar_t>;
    std::wstring_convert<convert_type, wchar_t> converter;
    std::string converted_str = converter.to_bytes(wide_string);
    return converted_str.c_str();
}

std::wstring utility::cstr_to_wstring(const char * c_str)
{
    std::string intermediate(c_str);
    std::wstring temp(intermediate.length(), L' ');
    std::copy(intermediate.begin(), intermediate.end(), temp.begin());
    return temp;
}
