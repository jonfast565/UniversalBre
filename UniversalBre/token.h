#pragma once

#include "token_types.h"

namespace core {
    class token
    {
    private:
        token_type _token_type;
        std::wstring _lexeme;
    public:
        explicit token(token_type type) : _token_type(type) {};
        explicit token(token_type type, std::wstring lexeme) : _token_type(type), _lexeme(lexeme) {};
        virtual ~token() {};

        token_type get_type() { return _token_type; }
        std::wstring get_lexeme() { return _lexeme; }
    };
}

