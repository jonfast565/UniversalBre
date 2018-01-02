#pragma once

#include <iostream>
#include <string>

namespace core {
    class console_handler {
    public:
        console_handler() {}
        virtual ~console_handler() {}

        std::wstring get_interactive_input() {
            std::wcout << L"> ";
            std::wstring input;
            std::wcin >> input;
            return input;
        }

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