#include "binop_expression_node.h"

void core::binop_expression_node::print(int indent)
{
    if (_left_node == nullptr) {
        std::wcout << utility::build_indent_str(indent) << " BINOP LEFT: NULL" << std::endl;
    }
    else {
        _left_node->print(indent + 1);
    }
    std::wcout << utility::build_indent_str(indent) << get_binop_type_string(_op_type) << std::endl;
    if (_right_node == nullptr) {
        std::wcout << utility::build_indent_str(indent) << " BINOP RIGHT: NULL" << std::endl;
    }
    else {
        _right_node->print(indent + 1);
    }
}

core::expression_node_ptr_s core::binop_expression_node::fold_nodes(
    expression_node_ptr_s left_node,
    expression_node_ptr_s right_node)
{
    if (left_node != nullptr && right_node != nullptr) {
        return utility::make_ptr_s(binop_expression_node(left_node, right_node, OP_INVALID));
    }
    if (left_node == nullptr && right_node != nullptr) {
        return right_node;
    }
    else if (left_node != nullptr && right_node == nullptr) {
        return left_node;
    }
    else {
        return nullptr;
    }
}

// It's important to remove all the nasty nulls before continuing to generate code
core::expression_node_ptr_s core::binop_expression_node::fold_nodes(
    expression_node_ptr_s left_node, 
    expression_node_ptr_s right_node,
    binop_type op_type)
{
    if (left_node != nullptr && right_node != nullptr) {
        return utility::make_ptr_s(binop_expression_node(left_node, right_node, op_type));
    }
    else if (left_node == nullptr && right_node != nullptr) {
        return right_node;
    }
    else if (left_node != nullptr && right_node == nullptr) {
        return left_node;
    }
    else {
        return nullptr;
    }
}

core::binop_type core::binop_expression_node::get_op_type()
{
    return _op_type;
}
