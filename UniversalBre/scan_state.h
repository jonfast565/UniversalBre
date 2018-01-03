#pragma once

#include "global_defines.h"
#include "exceptions.h"
#include "utility.h"

#include "token_types.h"
#include "token.h"

namespace core {
    class scan_state
    {
    private:
        // counters
        int _backtrack = 0;
        int _location = 0;
        int _line = 0;
        int _column = 0;

        // inputs
        std::wstring _input;
        std::wstring _input_left;

        // utilities
        void increment_location(int increment);
    public:
        scan_state(std::wstring input);
        scan_state(const scan_state& state);
        virtual ~scan_state();

        // utilities
        wchar_t get_char();
        wchar_t get_char(int offset);
        void skip_whitespace();

        // scan methods
        token scan_integer_literal();
        token scan_string_literal();
        token scan_identifier();
        token scan_plus_operator();
        token scan_minus_operator();
        token scan_multiply_operator();
        token scan_divide_operator();
        token scan_float_literal();
    };
}

