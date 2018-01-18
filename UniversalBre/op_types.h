#pragma once

#include "global_defines.h"

namespace core {
    enum class op_type {
        // binary operations
        OP_ADDITION,
        OP_SUBTRACTION,
        OP_MULTIPLICATION,
        OP_DIVISION,

        // jumps and breaks
        OP_JUMP_UNCONDITIONAL,
        OP_BREAK,

        // the default
        OP_INVALID
    };

    static std::map<op_type, std::wstring> op_type_index = {
        { op_type::OP_ADDITION, L"Add" },
        { op_type::OP_SUBTRACTION, L"Subtract" },
        { op_type::OP_MULTIPLICATION, L"Multiply" },
        { op_type::OP_DIVISION, L"Divide" },

        { op_type::OP_JUMP_UNCONDITIONAL , L"Jump Unconditional" },
        { op_type::OP_BREAK, L"Break" },

        { op_type::OP_INVALID, L"Invalid" }
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_op_type_string(op_type type) {
        return op_type_index[type];
    }
}