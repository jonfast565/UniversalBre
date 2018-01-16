#include "binop_expression_node.h"

void core::binop_expression_node::print(int indent)
{
    if (_left_node == nullptr) {
        std::wcout << utility::build_indent_str(indent) << " BINOP LEFT: NULL" << std::endl;
    }
    else {
        _left_node->print(++indent);
    }
    if (_right_node == nullptr) {
        std::wcout << utility::build_indent_str(indent) << " BINOP RIGHT: NULL" << std::endl;
    }
    else {
        _right_node->print(++indent);
    }
}
