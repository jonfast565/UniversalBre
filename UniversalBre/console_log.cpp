#include "console_log.h"

core::console_log::console_log(const bool debug) : log(debug)
{
}

void core::console_log::log_default(const std::wstring& message)
{
#ifdef _WIN32
    const auto h_console = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(h_console, WIN_BLUE);
    std::wcout << "[Info] " << message << std::endl;
    SetConsoleTextAttribute(h_console, WIN_WHITE);

#else
    std::wcout << "[Info] " << message << std::endl;
#endif
}

void core::console_log::log_warning(const std::wstring& warn_message)
{
#ifdef _WIN32
    const auto h_console = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(h_console, WIN_YELLOW);
    std::wcout << "[Warning] " << warn_message << std::endl;
    SetConsoleTextAttribute(h_console, WIN_WHITE);
#else
    std::wcerr << "[Warning] " << warn_message << std::endl;
#endif
}

void core::console_log::log_success(const std::wstring& success_message)
{
    if (_debug_mode)
    {
#ifdef _WIN32
        const auto h_console = GetStdHandle(STD_OUTPUT_HANDLE);
        SetConsoleTextAttribute(h_console, WIN_GREEN);
        std::wcout << "[Success] " << success_message << std::endl;
        SetConsoleTextAttribute(h_console, WIN_WHITE);
#else
        std::wcerr << "[Success] " << error_message << std::endl;
#endif
    }
}

void core::console_log::log_error(const std::wstring& error_message)
{
#ifdef _WIN32
    const auto h_console = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(h_console, WIN_RED);
    std::wcout << "[Error] " << error_message << std::endl;
    SetConsoleTextAttribute(h_console, WIN_WHITE);
#else
    std::wcerr << "[Error] " << error_message << std::endl;
#endif
}

void core::console_log::just_log(const std::wstring& message)
{
#ifdef _WIN32
    const auto h_console = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(h_console, WIN_WHITE);
    std::wcout << message << std::endl;
    SetConsoleTextAttribute(h_console, WIN_WHITE);
#else
    std::wcerr << error_message << std::endl;
#endif
}

void core::console_log::log_debug(const std::wstring& message)
{
    if (_debug_mode)
    {
#ifdef _WIN32
        const auto h_console = GetStdHandle(STD_OUTPUT_HANDLE);
        SetConsoleTextAttribute(h_console, WIN_WHITE);
        std::wcout << "[Debug] " << message << std::endl;
        SetConsoleTextAttribute(h_console, WIN_WHITE);
#else
        std::wcerr << "[Debug] " << message << std::endl;
#endif
    }
}

void core::console_log::set_console_font()
{
#ifdef _WIN32
    CONSOLE_FONT_INFOEX cfi;
    cfi.cbSize = sizeof(cfi);
    cfi.nFont = 0;
    cfi.dwFontSize.X = 0;
    cfi.dwFontSize.Y = 16;
    cfi.FontFamily = FF_DONTCARE;
    cfi.FontWeight = FW_NORMAL;
    wcsncpy_s(cfi.FaceName, DEFAULT_FONT, wcslen(DEFAULT_FONT));
    SetCurrentConsoleFontEx(GetStdHandle(STD_OUTPUT_HANDLE), FALSE, &cfi);
#else
    // TODO: IMPLEMENT MAC/LINUX SPECIFIC CODE HERE
#endif
}
