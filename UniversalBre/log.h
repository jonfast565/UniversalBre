#pragma once

#include "global_defines.h"

namespace core {
    class log
    {
    public:
        // ctor/dtor
        log() {}
        virtual ~log() {}

        // logging a warning
        virtual void log_warning(const std::wstring& warn_message) = 0;
        // an error
        virtual void log_error(const std::wstring& error_message) = 0;
        // info
        virtual void log_default(const std::wstring& message) = 0;
        // whatever
        virtual void just_log(const std::wstring& message) = 0;
        // TODO: Move platform specific method to global inline method
        virtual void set_console_font() = 0;
    };
}

