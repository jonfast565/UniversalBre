#pragma once

#include <string>

namespace core {
    class log
    {
    public:
        log() {}
        virtual ~log() {}
        virtual void log_warning(const std::wstring& warn_message) = 0;
        virtual void log_error(const std::wstring& error_message) = 0;
        virtual void log_default(const std::wstring& message) = 0;
        virtual void just_log(const std::wstring& message) = 0;
        virtual void set_console_font() = 0;
    };
}

