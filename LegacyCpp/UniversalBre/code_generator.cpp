#include "code_generator.h"

core::ssa_instruction_vecptrptr_s core::code_generator::generate_code()
{
    auto code = utility::make_ptr_s(ssa_instruction_ptr_vec_s());
    auto instruction = utility::make_ptr_s(
        ssa_instruction(op_type::op_addition, L"1", L"1", L"rax"));
    code->push_back(instruction);
    return code;
}
