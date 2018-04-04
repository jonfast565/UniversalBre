#pragma once

#include "global_defines.h"
#include "expression_node.h"
#include "utility.h"
#include "ssa_instruction.h"

namespace core
{
    class code_generator
    {
    public:
        core::ssa_instruction_vecptrptr_s generate_code();
    };
    ALIAS_TYPES(code_generator)
}
