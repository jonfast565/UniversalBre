#pragma once

#include "global_defines.h"
#include "expression_node.h"

#include "argument_list_node.h"

namespace core
{
    class function_expression_node
    {
    private:
        argument_list_node_ptr_s _argument_list;
    public:
        function_expression_node()
        {
            _argument_list = utility::make_ptr_s(argument_list_node());
        }

        virtual ~function_expression_node()
        {
        }

        argument_list_node_ptr_s get_argument_list();
        void set_argument_list(argument_list_node_ptr_s argument_list_node);
    };

    ALIAS_TYPES(function_expression_node)
}
