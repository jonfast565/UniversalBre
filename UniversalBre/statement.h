#pragma once

#include "global_defines.h"
#include "statement_types.h"

namespace core
{
    class statement
    {
        statement_type type_;
    public:
        explicit statement(statement_type type);
        statement_type get_type() const;
    };
    ALIAS_TYPES(statement)
}
