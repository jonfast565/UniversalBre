#include "singleop_expression_node.h"

void core::singleop_expression_node::print(int indent)
{
    if (_single_node == nullptr) {
        std::wcout << utility::build_indent_str(indent) << " SINGLEOP: NULL" << std::endl;
    }
    else {
        _single_node->print(indent);
    }
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
