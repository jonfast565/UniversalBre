#include "code_generator.h"

core::code_generator::code_generator()
{
}

core::ssa_instruction_vecptrptr_s core::code_generator::generate_code(
    expression_node_ptr_s starting_node)
{
    auto code = utility::make_ptr_s(std::vector<std::shared_ptr<ssa_instruction>>());
    auto instruction = utility::make_ptr_s(ssa_instruction(op_type::op_addition, L"1", L"1", L"rax"));
    code->push_back(instruction);
    return code;
}
