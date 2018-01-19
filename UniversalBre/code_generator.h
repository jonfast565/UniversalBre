#pragma once

#include "expression_node.h"
#include "utility.h"
#include "ssa_instruction.h"

namespace core {
    class code_generator
    {
    public:
        code_generator() {}
        ~code_generator() {}
        ssa_instruction_vecptrptr_s generate_code(expression_node_ptr_s starting_node);
    };
}

