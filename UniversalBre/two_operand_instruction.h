#pragma once

#include "instruction.h"
#include "binop_types.h"

namespace core {
    class two_operand_instruction
        : public instruction
    {
    private:
        binop_type _operator;
        std::wstring _operand1;
        std::wstring _operand2;
    public:
        two_operand_instruction(binop_type op, std::wstring operand1, std::wstring operand2) : 
            _operator(op), 
            _operand1(operand1), 
            _operand2(operand2) {}
        virtual ~two_operand_instruction() {}
        std::wstring get_code();
        void print_code();
    };
    PTR_ALIAS(two_operand_instruction)
}

