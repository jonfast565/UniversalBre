#pragma once

#include "global_defines.h"
#include "semantic_types.h"
#include "function_expression_node.h"
#include "statement_node.h"

namespace core
{
<<<<<<< HEAD
    class assignment_node : 
    public statement_node
=======
    class assignment_node : statement
>>>>>>> f77f72730c56a068b0aacda328b7d689a06ebc42
    {
    private:
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

        virtual ~assignment_node() = default;
    };

    ALIAS_TYPES(assignment_node)
}
