#include "stdafx.h"
#include "console_log.h"

void core::console_log::log_default(const std::wstring& message) {
#ifdef _WIN32
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, WIN_BLUE);
    std::wcout << "[Info] " << message << std::endl;
    SetConsoleTextAttribute(hConsole, WIN_WHITE);

#else
    std::wcout << "[Info] " << message << std::endl;
#endif
}

void core::console_log::log_warning(const std::wstring& warn_message) {
#ifdef _WIN32
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, WIN_YELLOW);
    std::wcout << "[Warning] " << warn_message << std::endl;
    SetConsoleTextAttribute(hConsole, WIN_WHITE);
#else
    std::wcerr << "[Warning] " << warn_message << std::endl;
#endif
}

void core::console_log::log_error(const std::wstring& error_message) {
#ifdef _WIN32
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, WIN_RED);
    std::wcout << "[Error] " << error_message << std::endl;
    SetConsoleTextAttribute(hConsole, WIN_WHITE);
#else
    std::wcerr << "[Error] " << error_message << std::endl;
#endif
}

void core::console_log::just_log(const std::wstring& message) {
#ifdef _WIN32
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, WIN_WHITE);
    std::wcout << message << std::endl;
    SetConsoleTextAttribute(hConsole, WIN_WHITE);
#else
    std::wcerr << error_message << std::endl;
#endif
}
