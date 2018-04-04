#pragma once

#include "global_defines.h"

namespace core
{
    class atom_status
    {
        const wchar_t atom_;
        bool is_digit_;
        bool is_whitespace_;
        bool is_alpha_;
        bool is_alpha_digit_;
        bool is_alpha_digit_underscore_;
        bool is_break_char_;
        bool is_integer_break_char_;
    public:
        explicit atom_status(wchar_t atom);
        bool is_dot() const;
        bool is_digit() const;
        bool is_whitespace() const;
        bool is_break_char() const;
        bool is_integer_break_char() const;
        bool is_alpha() const;
        bool is_alpha_digit() const;
        bool is_alpha_digit_underscore() const;
        bool is_empty() const;
        bool is_empty_or_whitespace() const;
        bool breaks_any() const;
        bool breaks_any_integer() const;
        bool breaks_any_string() const;
        bool is_underscore() const;
        bool is_identifier_char() const;
        wchar_t get_atom() const;
    };

    ALIAS_TYPES(atom_status)
}
