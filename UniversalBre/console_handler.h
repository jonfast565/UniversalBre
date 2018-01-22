#pragma once

#include "global_defines.h"

namespace core
{
    class console_handler
    {
    public:
        // for a single interactive input
        static std::wstring get_interactive_input();

        // for multiple interactive inputs
        // that two newlines terminate
        std::wstring get_interactive_multiple_input() const;
    };

    ALIAS_TYPES(console_handler)
}
