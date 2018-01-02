#pragma once

#include <stdio.h>
#include <wchar.h>
#include <wctype.h>

class utility {
public:
    static bool is_digit(const wchar_t possible_digit);
    static bool is_alpha(const wchar_t possible_alpha);
    static bool is_alpha_digit(const wchar_t possible_alpha_digit);
};