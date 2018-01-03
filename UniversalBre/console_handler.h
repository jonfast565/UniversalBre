#pragma once

#include "global_defines.h"

namespace core {
    class console_handler {
    public:
        // ctor/dtor
        console_handler() {}
        virtual ~console_handler() {}

        // for a single interactive input
        std::wstring get_interactive_input() {
            std::wcout << L"> ";
            std::wstring input;
            std::wcin >> input;
            return input;
        }

        // for multiple interactive inputs
        // that two newlines terminate
        std::wstring get_interactive_multiple_input() {
            std::wstring result;
            std::wstring input;
            do {
                std::cout << "> ";
                std::getline(std::wcin, input);
                if (!input.empty()) {
                    result += input + L"\n";
                }
            } while (!input.empty());
            return result;
        }
    };
}