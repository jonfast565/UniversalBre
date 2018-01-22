#include "function_expression_node.h"

core::argument_list_node_ptr_s core::function_expression_node::get_argument_list()
{
    return _argument_list;
}

void core::function_expression_node::set_argument_list(argument_list_node_ptr_s argument_list_node)
{
    _argument_list = argument_list_node;
}
