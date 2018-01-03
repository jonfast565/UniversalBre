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
    public:
        atom_status(const wchar_t atom);
        bool is_digit();
        bool is_whitespace();
        bool is_alpha();
        bool is_alpha_digit();
        bool is_alpha_digit_underscore();
        bool is_empty();
    };
}