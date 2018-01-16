#include "literal_expression_node.h"

void core::literal_expression_node::print(int indent)
{
    std::wcout << utility::build_indent_str(indent) << _value << std::endl;
}
