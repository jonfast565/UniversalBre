#pragma once

#include "global_defines.h"
#include "op_types.h"

namespace core
{
    class ssa_instruction
    {
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
            _target(target)
        {
        }

        virtual ~ssa_instruction()
        {
        }
    };

    ALIAS_TYPES(ssa_instruction)
}
