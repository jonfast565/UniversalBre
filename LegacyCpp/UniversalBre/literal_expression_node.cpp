#include "literal_expression_node.h"

core::literal_expression_node::literal_expression_node(std::wstring& value):
    expression_node(node_type::node_type_literal),
    _value(value)
{
}

void core::literal_expression_node::print(int indent)
{
    std::wcout
        << indent << " "
        << utility::build_indent_str(indent)
        << _value
        << std::endl;
}
