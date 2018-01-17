#pragma once

#include "global_defines.h"
#include "expression_node.h"
#include "binop_expression_node.h"
#include "literal_expression_node.h"
#include "singleop_expression_node.h"

namespace core {
    class expression_pruner
    {
    public:
        expression_pruner() { }
        ~expression_pruner() { }
        expression_node_ptr_s prune(
            expression_node_ptr_s current_expression,
            expression_node_ptr_s expression_parent = nullptr);
    };
    PTR_ALIAS(expression_pruner)
}

