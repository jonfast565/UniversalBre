#pragma once

#include "global_defines.h"
#include "log.h"

namespace core {
    class console_log :
        public log
    {
    public:
        // ctor/dtor
        console_log() : log() {};
        ~console_log() {};

        // same as described in interfaces
        void log_warning(const std::wstring& warn_message);
        void log_error(const std::wstring& error_message);
        void log_default(const std::wstring& message);
        void just_log(const std::wstring& message);
        void set_console_font();
    };
}

