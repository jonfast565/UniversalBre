#include "code_generator.h"

core::two_operand_instruction_vecptr_s core::code_generator::generate_code(
    expression_node_ptr_s starting_node)
{
    auto code = utility::make_ptr_s(std::vector<two_operand_instruction>());
    code->push_back(two_operand_instruction(OP_ADDITION, L"1", L"1"));
    return code;
}
