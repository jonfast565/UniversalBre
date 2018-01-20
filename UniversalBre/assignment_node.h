#pragma once

#include "global_defines.h"
#include "semantic_types.h"
#include "binop_expression_node.h"
#include "function_expression_node.h"

namespace core {
    class assignment_node
    {
    private:
        assignment_type _assignment_type;
        std::wstring _variable_name;
        expression_node_ptr_s _expression;
        function_expression_node_ptr_s _function_expression;
    public:
        explicit assignment_node(
            std::wstring variable_name, 
            expression_node_ptr_s expression) : 
            _variable_name(variable_name), 
            _expression(expression),
            _assignment_type(assignment_type::VARIABLE_ASSIGNMENT) {}
        explicit assignment_node(
            std::wstring variable_name,
            function_expression_node_ptr_s expression) :
            _variable_name(variable_name),
            _function_expression(expression),
            _assignment_type(assignment_type::FUNCTION_ASSIGNMENT) {}
        virtual ~assignment_node() {}
    };
    ALIAS_TYPES(assignment_node)
}

