#pragma once

#include "global_defines.h"
#include "utility.h"

namespace core {
    class expression_node {
    public:
        virtual void print(int indent) = 0;
    };
    PTR_ALIAS(expression_node)
}