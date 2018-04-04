#pragma once

#include "global_defines.h"
#include "semantic_types.h"
#include "function_expression_node.h"
#include "statement.h"
#include "expression_node.h"

namespace core
{
    class assignment_node : public statement
    {
        assignment_type assignment_type_;
        std::wstring variable_name_;
        expression_node_ptr_s expression_;
        function_expression_node_ptr_s function_expression_;
    public:
        explicit assignment_node(
            std::wstring variable_name,
            expression_node_ptr_s expression);

        explicit assignment_node(
            std::wstring variable_name,
            function_expression_node_ptr_s expression);
    };

    ALIAS_TYPES(assignment_node)
}
