#include "binop_expression_node.h"

void core::binop_expression_node::print(int indent)
{
    if (_left_node == nullptr) {
        std::wcout
            << indent
            << utility::build_indent_str(indent + 1) 
            << "N/A"
            << std::endl;
    }
    else {
        _left_node->print(indent + 1);
    }

    std::wcout
        << indent
        << utility::build_indent_str(indent)
        << get_binop_type_string(_op_type)
        << std::endl;

    if (_right_node == nullptr) {
        std::wcout 
            << indent
            << utility::build_indent_str(indent + 1) 
            << "N/A"
            << std::endl;
    }
    else {
        _right_node->print(indent + 1);
    }
}

void core::binop_expression_node::fold_expr_node()
{
    /*
    if (_right_node->get_node_type() == NODE_TYPE_SINGLE
        && _op_type == OP_EXPR_PART) {
        auto singleop_ptr =
            std::static_pointer_cast<singleop_expression_node>(_right_node);
        _op_type = singleop_ptr->get_op_type();
        _right_node = singleop_ptr->get_single_node();
    }

    if (_right_node->get_node_type() == NODE_TYPE_BINARY
        && _op_type == OP_EXPR_PART) {
        auto binaryop_ptr =
            std::static_pointer_cast<binop_expression_node>(_right_node);
        _op_type = binaryop_ptr->get_op_type();
        _right_node = binaryop_ptr;
    }
    */
}

core::binop_type core::binop_expression_node::get_op_type()
{
    return _op_type;
}

void core::binop_expression_node::set_op_type(binop_type type)
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
