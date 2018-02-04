#include "function_expression_node.h"

inline core::function_expression_node::function_expression_node() : 
    statement(statement_type::function_assignment_statement)
{
    _argument_list = utility::make_ptr_s(argument_list_node());
    _body_statements = utility::make_ptr_s(statement_vecptr_s());
}

core::statement_vecptrptr_s core::function_expression_node::get_body_statements()
{
    return _body_statements;
}

void core::function_expression_node::insert_body_statement(statement_ptr_s statement)
{
    _body_statements->push_back(statement);
}

core::argument_list_node_ptr_s core::function_expression_node::get_argument_list()
{
    return _argument_list;
}

void core::function_expression_node::set_argument_list(argument_list_node_ptr_s argument_list_node)
{
    _argument_list = argument_list_node;
}
