#pragma once

#include "global_defines.h"
#include "statement_types.h"

namespace core
{
    class statement_node
    {
        statement_type type_;
    public:
        explicit statement_node(statement_type type);
        statement_type get_type() const;
    };
    ALIAS_TYPES(statement_node)
}
