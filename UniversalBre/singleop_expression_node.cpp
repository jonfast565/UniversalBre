#include "singleop_expression_node.h"

void core::singleop_expression_node::print(int indent)
{
    if (_single_node == nullptr) {
        std::wcout << utility::build_indent_str(indent) << " SINGLEOP: NULL" << std::endl;
    }
    else {
        _single_node->print(++indent);
    }
}
