#pragma once

#include "global_defines.h"

namespace core {
    enum token_type {
        BEGIN_OF_FILE,
        INTEGER_LITERAL,
        STRING_LITERAL,
        PLUS_OPERATOR,
        MINUS_OPERATOR,
        MULTIPLY_OPERATOR,
        DIVIDE_OPERATOR,
        CONCAT_OPERATOR,
        END_OF_FILE,
        INVALID
    };

    static std::wstring get_type_string(token_type type) {
        switch (type) {
        case token_type::CONCAT_OPERATOR:
            break;
        }
    }
}