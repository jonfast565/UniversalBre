#pragma once

#include <iostream>
#include <memory>
#include "exceptions.h"
#include "utility.h"
#include "token_types.h"
#include "token.h"

namespace core {
    class scan_state
    {
    private:
        int _backtrack = 0;
        int _location = 0;
        int _line = 0;
        int _column = 0;
        std::wstring _input;
        std::wstring _input_left;
        void increment_location(int increment);
    public:
        scan_state(std::wstring input);
        scan_state(const scan_state& state);

        wchar_t get_char();
        wchar_t get_char(int offset);
        void skip_whitespace();
        token try_scan_integer();

        virtual ~scan_state();
    };
}

