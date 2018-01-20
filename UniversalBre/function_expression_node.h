#pragma once

#include "global_defines.h"
#include "expression_node.h"

namespace core {
    class function_expression_node {
    private:

    public:
        function_expression_node() {}
        virtual ~function_expression_node() {}
    };
    ALIAS_TYPES(function_expression_node)
}