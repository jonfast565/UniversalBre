#include "expression_pruner.h"

core::expression_node_ptr_s core::expression_pruner::prune(
    core::expression_node_ptr_s current_expression,
    core::expression_node_ptr_s parent_expression)
{
    // do not collapse null, we shouldn't be getting this far anyways
    if (current_expression == nullptr) {
        return nullptr;
    }

    auto current_expression_singleop = 
        std::static_pointer_cast<singleop_expression_node>(current_expression);

    // collapse single node into the node beneath it
    if (current_expression->get_node_type() == NODE_TYPE_SINGLE
        && current_expression_singleop != nullptr
        && parent_expression != nullptr) {
        auto current_child_node = current_expression_singleop->get_single_node();
        
        if (parent_expression->get_node_type() == NODE_TYPE_BINARY) {
            auto parent_expression_binop =
                std::static_pointer_cast<binop_expression_node>(parent_expression);
            parent_expression_binop->set_op_type(current_expression_singleop->get_op_type());
        }
        else if (parent_expression->get_node_type() == NODE_TYPE_SINGLE) {
            auto parent_expression_singleop =
                std::static_pointer_cast<singleop_expression_node>(parent_expression);
            parent_expression_singleop->set_op_type(current_expression_singleop->get_op_type());
        }

        auto new_singleop_node = prune(current_child_node, current_expression);
        return new_singleop_node;
    }

    auto current_expression_binop =
        std::static_pointer_cast<binop_expression_node>(current_expression);

    if (current_expression->get_node_type() == NODE_TYPE_BINARY
        && current_expression_binop != nullptr) {
        current_expression_binop->set_left_node(
            prune(current_expression_binop->get_left_node(), current_expression));
        current_expression_binop->set_right_node(
            prune(current_expression_binop->get_right_node(), current_expression));
        return current_expression_binop;
    }

    // if we didn't do anything, then don't mess with it
    return current_expression;
}
