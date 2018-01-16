#pragma once

#include "global_defines.h"

namespace core {
    enum token_type {
        // file markers
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

        // parenthesis
        LEFT_PARENTHESIS,
        RIGHT_PARENTHESIS,

        // error state
        // used for initialization only
        INVALID
    };

    static std::map<token_type, std::wstring> token_type_index = {
        // file markers
        { END_OF_FILE, L"End of file" },

        // literals
        { INTEGER_LITERAL, L"Integer literal" },
        { STRING_LITERAL, L"String literal" },
        { FLOAT_LITERAL, L"Float literal" },
        { IDENTIFIER, L"Identifier" },

        // operators
        { PLUS_OPERATOR, L"Plus operator" },
        { MINUS_OPERATOR, L"Minus operator" },
        { MULTIPLY_OPERATOR, L"Multiply operator" },
        { DIVIDE_OPERATOR, L"Divide operator" },
        { CONCAT_OPERATOR, L"Concat operator" },

        // parenthesis
        { LEFT_PARENTHESIS, L"Left parenthesis" },
        { RIGHT_PARENTHESIS, L"Right parenthesis" },

        // error state
        // used for initialization only
        { INVALID, L"Invalid" }
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_token_type_string(token_type type) {
        return token_type_index[type];
    }
}