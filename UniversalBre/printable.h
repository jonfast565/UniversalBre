#pragma once

#include "global_defines.h"

namespace core
{
    class printable
    {
    protected:
        ~printable() = default;
    public:
        virtual void print() = 0;
    };
    ALIAS_TYPES(printable)
}
