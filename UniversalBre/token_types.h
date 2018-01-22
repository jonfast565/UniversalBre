#pragma once

#include "global_defines.h"

namespace core
{
    enum class token_type
    {
        // file markers
        end_of_file,

        // scoping
        scope_begin_operator,
        scope_end_operator,

        // literals
        integer_literal,
        string_literal,
        float_literal,
        identifier,

        // operators
        plus_operator,
        minus_operator,
        multiply_operator,
        divide_operator,
        concat_operator,

        // boolean equality operators
        boolean_eq_operator,
        // equal to
        boolean_ne_operator,
        // not equal to

        // boolean comparison operators
        boolean_gt_operator,
        // greater than
        boolean_lt_operator,
        // less than
        boolean_lte_operator,
        // less than or equal to
        boolean_gte_operator,
        // greater than or equal to

        // boolean combinators
        boolean_and_operator,
        boolean_or_operator,

        // parenthesis
        left_parenthesis,
        right_parenthesis,

        // operators for special operations
        assignment_operator,

        // program delimiters
        semicolon,
        list_delimiter,

        // keywords
        function_keyword,
        infinite_keyword,
        break_keyword,
        feature_keyword,
        autobreak_keyword,
        on_keyword,
        off_keyword,

        // error state
        // used for initialization only
        invalid
    };

    static std::map<token_type, std::wstring> token_type_index = {
        // file markers
        {token_type::end_of_file, L"End of file"},

        // keywords
        {token_type::function_keyword, L"Function"},
        {token_type::list_delimiter, L"List delimiter"},

        // literals
        {token_type::integer_literal, L"Integer literal"},
        {token_type::string_literal, L"String literal"},
        {token_type::float_literal, L"Float literal"},
        {token_type::identifier, L"Identifier"},

        // operators
        {token_type::plus_operator, L"Plus operator"},
        {token_type::minus_operator, L"Minus operator"},
        {token_type::multiply_operator, L"Multiply operator"},
        {token_type::divide_operator, L"Divide operator"},
        {token_type::concat_operator, L"Concat operator"},

        // parenthesis
        {token_type::left_parenthesis, L"Left parenthesis"},
        {token_type::right_parenthesis, L"Right parenthesis"},
        {token_type::assignment_operator, L"Assignment operator"},

        // program delimiters
        {token_type::semicolon, L"Semicolon"},

        // error state
        // used for initialization only
        {token_type::invalid, L"Invalid"}
    };

    // TODO: IMPLEMENT COMPLETELY
    static std::wstring get_token_type_string(token_type type)
    {
        return token_type_index[type];
    }
}
