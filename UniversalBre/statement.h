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
<<<<<<< HEAD:UniversalBre/statement_node.h
    ALIAS_TYPES(statement_node)
=======
    ALIAS_TYPES(statement)
>>>>>>> f77f72730c56a068b0aacda328b7d689a06ebc42:UniversalBre/statement.h
}
