#pragma once

#include "global_defines.h"
#include "log.h"

namespace core
{
    class console_log :
        public log
    {
    public:
        explicit console_log(const bool debug);
        void log_warning(const std::wstring& warn_message) override;
        void log_success(const std::wstring& success_message) override;
        void log_error(const std::wstring& error_message) override;
        void log_default(const std::wstring& message) override;
        void just_log(const std::wstring& message) override;
        void log_debug(const std::wstring& message) override;
        void set_console_font() override;
    };
    ALIAS_TYPES(console_log)
}
