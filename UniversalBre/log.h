#pragma once

#include "global_defines.h"

namespace core {
    class log
    {
    protected:
        bool _debug_mode;
    public:
        // ctor/dtor
        log(bool debug_mode) : _debug_mode(debug_mode) {}
        virtual ~log() {}

        // logging a warning
        virtual void log_warning(const std::wstring& warn_message) = 0;
        // logging a debug success
        virtual void log_success(const std::wstring& success_message) = 0;
        // an error
        virtual void log_error(const std::wstring& error_message) = 0;
        // info
        virtual void log_default(const std::wstring& message) = 0;
        // whatever
        virtual void just_log(const std::wstring& message) = 0;
        // logging debug events, in case of errors that can be hidden
        virtual void log_debug(const std::wstring& message) = 0;
        // TODO: Move platform specific method to global inline method
        virtual void set_console_font() = 0;

        void set_debug(bool debug_mode) {
            _debug_mode = debug_mode;
        }
    };
    ALIAS_TYPES(log)
}

