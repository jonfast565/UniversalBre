#include "binop_expression_node.h"

void core::binop_expression_node::print(int indent)
{
    if (_left_node == nullptr) {
        std::wcout
            << indent << " "
            << utility::build_indent_str(indent + 1) 
            << "N/A"
            << std::endl;
    }
    else {
        _left_node->print(indent + 1);
    }

    std::wcout
        << indent << " "
        << utility::build_indent_str(indent)
        << get_op_type_string(_op_type)
        << std::endl;

    if (_right_node == nullptr) {
        std::wcout 
            << indent << " "
            << utility::build_indent_str(indent + 1) 
            << "N/A"
            << std::endl;
    }
    else {
        _right_node->print(indent + 1);
    }
}

core::op_type core::binop_expression_node::get_op_type()
{
    return _op_type;
}

void core::binop_expression_node::set_op_type(op_type type)
{
    _op_type = type;
}

bool core::binop_expression_node::one_node_populated()
{
    return _left_node == nullptr
        && _right_node != nullptr
        || _left_node != nullptr
        && _right_node == nullptr;
}

bool core::binop_expression_node::two_nodes_populated()
{
    return _left_node != nullptr && _right_node != nullptr;
}

bool core::binop_expression_node::left_node_populated()
{
    return _left_node != nullptr;
}

bool core::binop_expression_node::right_node_populated()
{
    return _right_node != nullptr;
}

core::expression_node_ptr_s core::binop_expression_node::get_left_node()
{
    assert(_left_node != nullptr);
    return _left_node;
}

core::expression_node_ptr_s core::binop_expression_node::get_right_node()
{
    assert(_right_node != nullptr);
    return _right_node;
}

core::expression_node_ptr_s core::binop_expression_node::get_populated_node()
{
    assert(one_node_populated());
    if (left_node_populated()) {
        return _left_node;
    }
    if (right_node_populated()) {
        return _right_node;
    }

    // SHOULD NEVER HAPPEN (assert)
    return nullptr;
}

void core::binop_expression_node::set_left_node(expression_node_ptr_s node)
{
    _left_node = node;
}

void core::binop_expression_node::set_right_node(expression_node_ptr_s node)
{
    _right_node = node;
}
