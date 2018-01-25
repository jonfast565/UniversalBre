#include "assignment_node.h"

core::assignment_node::assignment_node(
    std::wstring variable_name, 
    expression_node_ptr_s expression):
        statement_node(statement_type::assignment_statement),
        assignment_type_(assignment_type::variable_assignment),
        variable_name_(std::move(variable_name)),
        expression_(std::move(expression))
{
}

core::assignment_node::assignment_node(
    std::wstring variable_name, 
    function_expression_node_ptr_s expression):
        statement_node(statement_type::assignment_statement),
        assignment_type_(assignment_type::function_assignment),
        variable_name_(std::move(variable_name)),
        function_expression_(std::move(expression))
{
}
