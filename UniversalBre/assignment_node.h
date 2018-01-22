#pragma once

#include <utility>
#include "global_defines.h"
#include "semantic_types.h"
#include "binop_expression_node.h"
#include "function_expression_node.h"

namespace core
{
    class assignment_node
    {
    private:
        assignment_type assignment_type_;
        std::wstring variable_name_;
        expression_node_ptr_s expression_;
        function_expression_node_ptr_s function_expression_;
    public:
        explicit assignment_node(
            std::wstring variable_name,
            expression_node_ptr_s expression) :
            assignment_type_(assignment_type::variable_assignment),
            variable_name_(std::move(variable_name)),
            expression_(std::move(expression))
        {
        }

        explicit assignment_node(
            std::wstring variable_name,
            function_expression_node_ptr_s expression) :
            assignment_type_(assignment_type::function_assignment),
            variable_name_(std::move(variable_name)),
            function_expression_(std::move(expression))
        {
        }

        virtual ~assignment_node() = default;
    };

    ALIAS_TYPES(assignment_node)
}
