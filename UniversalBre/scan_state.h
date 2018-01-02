#pragma once

#include <iostream>
#include <memory>
#include "exceptions.h"
#include "utility.h"

namespace core {
    class scan_state
    {
    private:
        int _backtrack = 0;
        int _location = 0;
        int _line = 0;
        int _column = 0;
        std::wstring _input;
        void increment_location(int increment);
    public:
        scan_state(std::wstring input);
        scan_state(const scan_state& state);

        wchar_t get_char();
        void skip_whitespace();
        void scan_integer();

        virtual ~scan_state();
    };

    using ref_scan_state = std::unique_ptr<core::scan_state>;
}

