#pragma once

#include "global_defines.h"

namespace core {
    class console_waiter {
    public:
        console_waiter() {}
        virtual ~console_waiter() {}
        void wait();
    };
    PTR_ALIAS(console_waiter)
}