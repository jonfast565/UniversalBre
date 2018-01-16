#include "expression_pruner.h"

core::expression_node_ptr_s core::expression_pruner::prune(
    core::expression_node_ptr_s current_expression, 
    core::expression_node_ptr_s expression_parent)
{
    if (current_expression == nullptr) {
        return nullptr;
    }

    return current_expression;
}
