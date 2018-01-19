#pragma once

#include "global_defines.h"
#include "utility.h"

namespace core {
    class atom_status
    {
    private:
        const wchar_t _atom;
        bool _is_digit;
        bool _is_whitespace;
        bool _is_alpha;
        bool _is_alpha_digit;
        bool _is_alpha_digit_underscore;
        bool _is_break_char;
        bool _is_integer_break_char;
    public:
        atom_status(const wchar_t atom);
        bool is_dot();
        bool is_digit();
        bool is_whitespace();
        bool is_break_char();
        bool is_integer_break_char();
        bool is_alpha();
        bool is_alpha_digit();
        bool is_alpha_digit_underscore();
        bool is_empty();
        bool is_empty_or_whitespace();
        bool breaks_any();
        bool breaks_any_integer();
        bool breaks_any_string();
        bool is_underscore();
        bool is_identifier_char();
        wchar_t get_atom();
    };
    PTR_ALIAS(atom_status)
}