#pragma once

#include "expression_node.h"
#include "instruction.h"
#include "utility.h"
#include "two_operand_instruction.h"

namespace core {
    class code_generator
    {
    public:
        code_generator() {}
        ~code_generator() {}
        two_operand_instruction_vecptr_s generate_code(expression_node_ptr_s starting_node);
    };
}

