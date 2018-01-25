#pragma once

#include "global_defines.h"
#include "expression_node.h"

#include "argument_list_node.h"
#include "statement_node.h"

namespace core
{
    class function_expression_node : 
    public statement_node
    {
        argument_list_node_ptr_s argument_list_;
    public:
        function_expression_node() : 
        statement_node(statement_type::function_assignment_statement)
        {
            argument_list_ = utility::make_ptr_s(argument_list_node());
        }

        virtual ~function_expression_node()
        {
            argument_list_.reset();
        }

        argument_list_node_ptr_s get_argument_list();
        void set_argument_list(argument_list_node_ptr_s argument_list_node);
    };

    ALIAS_TYPES(function_expression_node)
}
