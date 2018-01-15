#pragma once

#include "global_defines.h"
#include "log.h"

namespace core {
    class console_log :
        public log
    {
    private:
        bool _debugging_enabled = false;
    public:
        // ctor/dtor
        console_log(bool debug) : log(), _debugging_enabled(debug) {};
        ~console_log() {};

        // same as described in interfaces
        void log_warning(const std::wstring& warn_message);
        void log_success(const std::wstring& success_message);
        void log_error(const std::wstring& error_message);
        void log_default(const std::wstring& message);
        void just_log(const std::wstring& message);
        void log_debug(const std::wstring& message);
        void set_console_font();
    };
    PTR_ALIAS(console_log)
}

