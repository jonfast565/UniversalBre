#include "token.h"

core::token::token(const token_type type) : token_type_(type)
{
}

core::token::token(const token_type type, std::wstring lexeme)
    : token_type_(type), lexeme_(std::move(lexeme))
{
}

core::token_type core::token::get_type() const
{
    return token_type_;
}

std::wstring core::token::get_lexeme() const
{
    return lexeme_;
}
