#pragma once

#include "global_defines.h"
#include "token_types.h"

namespace core {
    class token
    {
    private:
        token_type _token_type;
        std::wstring _lexeme;
    public:
        explicit token(token_type type);
        explicit token(token_type type, std::wstring lexeme);
        virtual ~token();

        token_type get_type();
        std::wstring get_lexeme();
    };
    ALIAS_TYPES(token)
}

