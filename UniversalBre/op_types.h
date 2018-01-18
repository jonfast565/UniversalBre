#pragma once

#include "global_defines.h"

namespace core {
    enum op_type {
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
        { OP_ADDITION, L"Add" },
        { OP_SUBTRACTION, L"Subtract" },
        { OP_MULTIPLICATION, L"Multiply" },
        { OP_DIVISION, L"Divide" },

        { OP_JUMP_UNCONDITIONAL , L"Jump Unconditional" },
        { OP_BREAK, L"Break" },

        { OP_INVALID, L"Invalid" }
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_op_type_string(op_type type) {
        return op_type_index[type];
    }
}