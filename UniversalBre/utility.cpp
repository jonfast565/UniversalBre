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
