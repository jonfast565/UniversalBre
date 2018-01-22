#pragma once

#include "statement_types.h"
#include "ssa_instruction.h"

namespace core {
    class statement
    {
    private:
        statement_type _type;

    public:
        statement() {}
        virtual ~statement() {}
    };
}

