#pragma once

#include "global_defines.h"

namespace core {
    enum token_type {
        // file markers
        BEGIN_OF_FILE,
        END_OF_FILE,

        // literals
        INTEGER_LITERAL,
        STRING_LITERAL,
        FLOAT_LITERAL,
        IDENTIFIER,

        // operators
        PLUS_OPERATOR,
        MINUS_OPERATOR,
        MULTIPLY_OPERATOR,
        DIVIDE_OPERATOR,
        CONCAT_OPERATOR,

        // error state
        // used for initialization only
        INVALID
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_type_string(token_type type) {
        switch (type) {
        case token_type::CONCAT_OPERATOR:
            break;
        }
    }
}