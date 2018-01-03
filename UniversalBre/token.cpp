#include "token.h"

core::token::token(token_type type) : _token_type(type)
{
}

core::token::token(token_type type, std::wstring lexeme)
    : _token_type(type), _lexeme(lexeme)
{

}

core::token::~token()
{
}

core::token_type core::token::get_type()
{
    return _token_type;
}

std::wstring core::token::get_lexeme()
{
    return _lexeme;
}
