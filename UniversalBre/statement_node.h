#pragma once

#include "statement_types.h"
#include "function_expression_node.h"
#include "ssa_instruction.h"

namespace core
{
    class statement
    {
        statement_type type_;
    public:
        statement(statement_type type);
        statement_type get_type() const;
    };
}
