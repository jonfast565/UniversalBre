#pragma once

#include <iostream>
#include "exceptions.h"

namespace core {
    class scan_state
    {
    private:
        int _backtrack = 0;
        int _location = 0;
        int _line = 0;
        int _column = 0;
        std::wstring _input;
        void increment_one();
        void increment_one_line();
    public:
        scan_state(std::wstring input);
        scan_state(const scan_state& state);

        wchar_t get_char();
        void try_scan_integer();

        virtual ~scan_state();
    };
}

