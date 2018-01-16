#include "two_operand_instruction.h"

std::wstring core::two_operand_instruction::get_code()
{
    return get_binop_type_string(_operator) + L" " + _operand1 + L", " + _operand2;
}

void core::two_operand_instruction::print_code()
{
    std::wcout << get_code() << std::endl;
}
