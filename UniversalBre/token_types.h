#pragma once

#include "global_defines.h"

namespace core {
    enum class token_type {
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

        // boolean equality operators
        BOOLEAN_EQ_OPERATOR, // equal to
        BOOLEAN_NE_OPERATOR, // not equal to

        // boolean comparison operators
        BOOLEAN_GT_OPERATOR, // greater than
        BOOLEAN_LT_OPERATOR, // less than
        BOOLEAN_LTE_OPERATOR, // less than or equal to
        BOOLEAN_GTE_OPERATOR, // greater than or equal to

        // boolean combinators
        BOOLEAN_AND_OPERATOR,
        BOOLEAN_OR_OPERATOR,

        // parenthesis
        LEFT_PARENTHESIS,
        RIGHT_PARENTHESIS,

        // operators for special operations
        ASSIGNMENT_OPERATOR,

        // program delimiters
        SEMICOLON,

        // error state
        // used for initialization only
        INVALID
    };

    static std::map<token_type, std::wstring> token_type_index = {
        // file markers
        { token_type::END_OF_FILE, L"End of file" },

        // literals
        { token_type::INTEGER_LITERAL, L"Integer literal" },
        { token_type::STRING_LITERAL, L"String literal" },
        { token_type::FLOAT_LITERAL, L"Float literal" },
        { token_type::IDENTIFIER, L"Identifier" },

        // operators
        { token_type::PLUS_OPERATOR, L"Plus operator" },
        { token_type::MINUS_OPERATOR, L"Minus operator" },
        { token_type::MULTIPLY_OPERATOR, L"Multiply operator" },
        { token_type::DIVIDE_OPERATOR, L"Divide operator" },
        { token_type::CONCAT_OPERATOR, L"Concat operator" },

        // parenthesis
        { token_type::LEFT_PARENTHESIS, L"Left parenthesis" },
        { token_type::RIGHT_PARENTHESIS, L"Right parenthesis" },

        { token_type::ASSIGNMENT_OPERATOR, L"Assignment operator" },

        // program delimiters
        { token_type::SEMICOLON, L"Semicolon" },

        // error state
        // used for initialization only
        { token_type::INVALID, L"Invalid" }
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_token_type_string(token_type type) {
        return token_type_index[type];
    }
}