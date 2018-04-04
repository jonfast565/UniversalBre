#pragma once

#include "global_defines.h"

namespace core
{
    enum class op_type
    {
        // binary math operations
        op_addition,
        op_subtraction,
        op_multiplication,
        op_division,

        // boolean equality operations
        op_boolean_eq,
        // equal to
        op_boolean_ne,
        // not equal to

        // boolean comparison operations
        op_boolean_gt,
        // greater than
        op_boolean_lt,
        // less than
        op_boolean_lte,
        // less than or equal to
        op_boolean_gte,
        // greater than or equal to

        // boolean combinators
        op_boolean_and,
        op_boolean_or,

        // other binary operations
        op_concat_strings,

        // jumps and breaks
        op_jump_unconditional,
        op_break,

        // special
        op_assign,

        // the default
        op_invalid
    };

    static std::map<op_type, std::wstring> op_type_index = {
        {op_type::op_addition, L"Add"},
        {op_type::op_subtraction, L"Subtract"},
        {op_type::op_multiplication, L"Multiply"},
        {op_type::op_division, L"Divide"},

        {op_type::op_concat_strings, L"String Concatenation"},

        // boolean equality operations
        {op_type::op_boolean_eq, L"Boolean Equals"}, // equal to
        {op_type::op_boolean_ne, L"Boolean Not-Equals"}, // not equal to

        // boolean comparison operations
        {op_type::op_boolean_gt, L"Boolean Greater Than"}, // greater than
        {op_type::op_boolean_lt, L"Boolean Less Than"}, // less than
        {op_type::op_boolean_lte, L"Boolean Less Than Equals"}, // less than or equal to
        {op_type::op_boolean_gte, L"Boolean Greater Than Equals"}, // greater than or equal to

        // boolean combinators
        {op_type::op_boolean_and, L"Boolean And"},
        {op_type::op_boolean_or, L"Boolean Or"},

        {op_type::op_jump_unconditional, L"Jump Unconditional"},
        {op_type::op_break, L"Break"},

        {op_type::op_invalid, L"Invalid"}
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_op_type_string(op_type type)
    {
        return op_type_index[type];
    }
}
