#pragma once

#include "global_defines.h"
#include "instruction.h"
#include "op_types.h"

namespace core {
    class ssa_instruction
    {
    private:
        op_type _type;
        std::wstring _first_operand;
        std::wstring _second_operand;
        std::wstring _target;
    public:
        ssa_instruction(op_type type, 
        std::wstring first_operand,
        std::wstring second_operand,
        std::wstring target) :
            _type(type),
            _first_operand(first_operand), 
            _second_operand(second_operand), 
            _target(target) {}
        virtual ~ssa_instruction() {}
    };
    PTR_ALIAS(ssa_instruction)
}

