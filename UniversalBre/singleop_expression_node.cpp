#include "singleop_expression_node.h"

void core::singleop_expression_node::print(int indent)
{
    if (_single_node == nullptr) {
        std::wcout 
            << indent << " "
            << utility::build_indent_str(indent + 1) 
            << "N/A" 
            << std::endl;
    }
    else {
        std::wcout 
            << indent << " "
            << utility::build_indent_str(indent) 
            << get_op_type_string(_temp_op_type) 
            << std::endl;

        _single_node->print(indent + 1);
    }
}

core::op_type core::singleop_expression_node::get_op_type()
{
    return _temp_op_type;
}

void core::singleop_expression_node::set_op_type(op_type type)
{
    _temp_op_type = type;
}

core::expression_node_ptr_s core::singleop_expression_node::get_single_node()
{
    assert(_single_node != nullptr);
    return _single_node;
}

void core::singleop_expression_node::set_single_node(expression_node_ptr_s node)
{
    _single_node = node;
}
