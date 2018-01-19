#pragma once

#include "global_defines.h"

namespace core {
    enum class op_type {
        // binary math operations
        OP_ADDITION,
        OP_SUBTRACTION,
        OP_MULTIPLICATION,
        OP_DIVISION,

        // boolean equality operations
        OP_BOOLEAN_EQ, // equal to
        OP_BOOLEAN_NE, // not equal to

        // boolean comparison operations
        OP_BOOLEAN_GT, // greater than
        OP_BOOLEAN_LT, // less than
        OP_BOOLEAN_LTE, // less than or equal to
        OP_BOOLEAN_GTE, // greater than or equal to

        // boolean combinators
        OP_BOOLEAN_AND,
        OP_BOOLEAN_OR,

        // other binary operations
        OP_CONCAT_STRINGS,

        // jumps and breaks
        OP_JUMP_UNCONDITIONAL,
        OP_BREAK,

        // special
        OP_ASSIGN,

        // the default
        OP_INVALID
    };

    static std::map<op_type, std::wstring> op_type_index = {
        { op_type::OP_ADDITION, L"Add" },
        { op_type::OP_SUBTRACTION, L"Subtract" },
        { op_type::OP_MULTIPLICATION, L"Multiply" },
        { op_type::OP_DIVISION, L"Divide" },

        { op_type::OP_CONCAT_STRINGS, L"String Concatenation"},

        // boolean equality operations
        { op_type::OP_BOOLEAN_EQ, L"Boolean Equals" },// equal to
        { op_type::OP_BOOLEAN_NE, L"Boolean Not-Equals" },// not equal to

        // boolean comparison operations
        { op_type::OP_BOOLEAN_GT, L"Boolean Greater Than" },// greater than
        { op_type::OP_BOOLEAN_LT, L"Boolean Less Than" },// less than
        { op_type::OP_BOOLEAN_LTE, L"Boolean Less Than Equals" },// less than or equal to
        { op_type::OP_BOOLEAN_GTE, L"Boolean Greater Than Equals" },// greater than or equal to

        // boolean combinators
        { op_type::OP_BOOLEAN_AND, L"Boolean And" },
        { op_type::OP_BOOLEAN_OR, L"Boolean Or" },

        { op_type::OP_JUMP_UNCONDITIONAL , L"Jump Unconditional" },
        { op_type::OP_BREAK, L"Break" },

        { op_type::OP_INVALID, L"Invalid" }
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_op_type_string(op_type type) {
        return op_type_index[type];
    }
}