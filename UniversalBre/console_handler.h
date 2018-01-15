#pragma once

#include "global_defines.h"

namespace core {
    class console_handler {
    public:
        // ctor/dtor
        console_handler();
        virtual ~console_handler();

        // for a single interactive input
        std::wstring get_interactive_input();

        // for multiple interactive inputs
        // that two newlines terminate
        std::wstring get_interactive_multiple_input();
    };
    PTR_ALIAS(console_handler)
}