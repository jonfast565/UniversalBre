#pragma once

#include "statement_types.h"

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
