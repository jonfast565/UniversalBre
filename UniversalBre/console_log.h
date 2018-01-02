#pragma once

#include "log.h"
#include <iostream>

#ifdef _WIN32
#include <windows.h>
#include "win_colors.h"
#include <string.h>  
#include <stdlib.h>  
#include <stdio.h>  
#include <errno.h>  
#define DEFAULT_FONT L"Courier"
#endif

namespace core {
    class console_log :
        public log
    {
    public:
        console_log() : log() {};
        ~console_log() {};
        void log_warning(const std::wstring& warn_message);
        void log_error(const std::wstring& error_message);
        void log_default(const std::wstring& message);
        void just_log(const std::wstring& message);
        void set_console_font();
    };
}

