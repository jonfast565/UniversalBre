#pragma once

#include "global_defines.h"
#include "binop_expression_node.h"

namespace core {
    class assignment_node
    {
    private:
        std::wstring _variable_name;
        expression_node_ptr_s _expression;
    public:
        assignment_node(
            std::wstring variable_name, 
            expression_node_ptr_s expression) : 
            _variable_name(variable_name), 
            _expression(expression) {}
        virtual ~assignment_node() {}
    };
    PTR_ALIAS(assignment_node)
}

