#include "console_handler.h"

std::wstring core::console_handler::get_interactive_input()
{
    std::wcout << L"> ";
    std::wstring input;
    std::wcin >> input;
    std::wcout << std::flush;
    return input;
}

std::wstring core::console_handler::get_interactive_multiple_input() const
{
    std::wstring result;
    std::wstring input;
    do
    {
        std::wcout << L"> ";
        std::getline(std::wcin, input);
        if (!input.empty())
        {
            result += input + L"\n";
        }
    }
    while (!input.empty());
    std::wcout << std::flush;
    return result;
}
