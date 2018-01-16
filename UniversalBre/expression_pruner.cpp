#include "expression_pruner.h"

core::expression_node_ptr_s core::expression_pruner::prune(
    core::expression_node_ptr_s current_expression)
{
    // do not collapse null, we shouldn't be getting this far anyways
    if (current_expression == nullptr) {
        return nullptr;
    }

    auto current_expression_singleop = 
        std::static_pointer_cast<singleop_expression_node>(current_expression);

    // collapse single node into the node beneath it
    if (current_expression->get_node_type() == NODE_TYPE_SINGLE
        && current_expression_singleop != nullptr) {
        auto current_child_node = current_expression_singleop->get_single_node();
        auto new_singleop_node = prune(current_child_node);
        return new_singleop_node;
    }

    auto current_expression_binop = 
        std::static_pointer_cast<binop_expression_node>(current_expression);

    // collapse binary nodes to single nodes if one side is null
    if (current_expression->get_node_type() == NODE_TYPE_BINARY
        && current_expression_binop != nullptr
        && current_expression_binop->one_node_populated()) {
        auto child_node = current_expression_binop->get_populated_node();
        auto singleop_ptr = utility::make_ptr_s(singleop_expression_node(child_node, current_expression_binop->get_op_type()));
        singleop_ptr->set_single_node(prune(singleop_ptr->get_single_node()));
        return singleop_ptr;
    }

    // if both are populated then prune the left and right children
    if (current_expression->get_node_type() == NODE_TYPE_BINARY
        && current_expression_binop != nullptr
        && current_expression_binop->two_nodes_populated()) {
        auto left_child_node = current_expression_binop->get_left_node();
        auto right_child_node = current_expression_binop->get_right_node();

        auto new_left_child_node = prune(left_child_node);
        auto new_right_child_node = prune(right_child_node);

        current_expression_binop->set_left_node(new_left_child_node);
        current_expression_binop->set_right_node(new_right_child_node);

        return current_expression_binop;
    }

    // if we didn't do anything, then don't mess with it
    return current_expression;
}
