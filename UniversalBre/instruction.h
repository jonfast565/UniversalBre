#pragma once

#include "global_defines.h"

namespace core {
    class instruction
    {
    public:
        virtual std::wstring get_code() = 0;
        virtual void print_code() = 0;
    };
    PTR_ALIAS(instruction)
}

