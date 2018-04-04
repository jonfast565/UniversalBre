#pragma once

#include "global_defines.h"
#include "token_types.h"

namespace core
{
    class token
    {
        token_type token_type_;
        std::wstring lexeme_;
    public:
        explicit token(token_type type);
        explicit token(token_type type, std::wstring lexeme);
        virtual ~token() = default;
        token_type get_type() const;
        std::wstring get_lexeme() const;
    };
    ALIAS_TYPES(token)
}
