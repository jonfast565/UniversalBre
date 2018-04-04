#pragma once

#include "global_defines.h"
#include "argument_list_node.h"
#include "statement.h"

namespace core
{
    class function_expression_node : public statement
    {
        argument_list_node_ptr_s _argument_list;
        statement_vecptrptr_s _body_statements;
    public:
        function_expression_node();
        statement_vecptrptr_s get_body_statements() const;
        void insert_body_statement(statement_ptr_s statement) const;
        argument_list_node_ptr_s get_argument_list() const;
        void set_argument_list(argument_list_node_ptr_s argument_list_node);
    };
    ALIAS_TYPES(function_expression_node)
}
