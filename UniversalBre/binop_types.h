#pragma once

#include "global_defines.h"

namespace core {
    enum binop_type {
        OP_ADDITION,
        OP_SUBTRACTION,
        OP_MULTIPLICATION,
        OP_DIVISION,
        OP_INVALID,
        OP_EXPR_PART
    };

    static std::map<binop_type, std::wstring> binop_type_index = {
        { OP_INVALID, L"Invalid" },
        { OP_ADDITION, L"Add" },
        { OP_SUBTRACTION, L"Subtract" },
        { OP_MULTIPLICATION, L"Multiply" },
        { OP_DIVISION, L"Divide" },
        { OP_EXPR_PART, L"Expr Part" }
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_binop_type_string(binop_type type) {
        return binop_type_index[type];
    }
}