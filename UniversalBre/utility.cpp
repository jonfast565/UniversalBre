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

bool utility::is_specific_char(
    const wchar_t current_char, 
    const wchar_t possible_specific_char, 
    bool case_invariant)
{
    if (!case_invariant) {
        return current_char == possible_specific_char;
    }
    return towlower(current_char) == towlower(possible_specific_char);
}

bool utility::is_charset(
    const wchar_t possible_charset, 
    const std::vector<wchar_t> charset)
{
    for (auto char_iter : charset) {
        if (possible_charset == char_iter)
            return true;
    }
    return false;
}

bool utility::is_not_charset(
    const wchar_t possible_charset, 
    const std::vector<wchar_t> charset)
{
    return !is_charset(possible_charset, charset);
}

bool utility::is_whitespace(const wchar_t possible_whitespace)
{
    return is_charset(possible_whitespace, 
        array_to_vector<wchar_t>(core::whitespace_chars, core::whitespace_chars_length));
}

bool utility::is_not_whitespace(const wchar_t possible_whitespace)
{
    return !is_whitespace(possible_whitespace);
}

bool utility::is_break_char(const wchar_t possible_break_char)
{
    return is_charset(possible_break_char, array_to_vector<wchar_t>(core::breaking_chars, core::breaking_chars_length));
}

const char * utility::wstring_to_cstr(std::wstring& wide_string)
{
    using convert_type = std::codecvt_utf8<wchar_t>;
    std::wstring_convert<convert_type, wchar_t> converter;
    std::string converted_str = converter.to_bytes(wide_string);
    return converted_str.c_str();
}

const wchar_t * utility::wstring_to_wcstr(std::wstring& wide_string)
{
    return wide_string.c_str();
}

std::wstring utility::cstr_to_wstring(const char * c_str)
{
    std::string intermediate(c_str);
    std::wstring temp(intermediate.length(), L' ');
    std::copy(intermediate.begin(), intermediate.end(), temp.begin());
    return temp;
}

std::wstring utility::build_indent_str(int indent)
{
    auto s = std::wstring();
    for (int i = 0; i < indent; i++) {
        s += L" - ";
    }
    return s;
}

const void utility::concat_in_place(const wchar_t ** result, const int count, ...)
{
    va_list list;
    va_start(list, count);
    std::wstring temp;
    for (int i = 0; i < count; i++) {
        auto argument = va_arg(list, const wchar_t *);
        temp += std::wstring(argument);
    }
    va_end(list);

    // Must free after using this method, srsly
    // this is a bad pattern, exposing the caller to memory mgmt
    // TODO: Implement something better
    *result = _wcsdup(temp.c_str());
}

const void utility::concat_two(
    const wchar_t ** result, 
    const wchar_t * one, 
    const wchar_t * two)
{
    concat_in_place(result, 2, one, two);
}

const void utility::concat_three(
    const wchar_t ** result, 
    const wchar_t * one, 
    const wchar_t * two, 
    const wchar_t * three)
{
    concat_in_place(result, 3, one, two, three);
}
