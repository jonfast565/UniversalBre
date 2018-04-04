#pragma once

#include "global_defines.h"

namespace core
{
    class console_waiter
    {
    public:
        console_waiter()
        {
        }

        virtual ~console_waiter()
        {
        }

        static void wait();
    };

    ALIAS_TYPES(console_waiter)
}
