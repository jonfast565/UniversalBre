#include "atom_status.h"
#include "utility.h"

core::atom_status::atom_status(const wchar_t atom) : atom_(atom)
{
    is_digit_ = utility::is_digit(atom);
    is_whitespace_ = utility::is_whitespace(atom);
    is_alpha_ = utility::is_alpha(atom);
    is_alpha_digit_ = utility::is_alpha_digit(atom);
    is_alpha_digit_underscore_ = utility::is_alpha_digit(atom) || is_underscore();
    is_break_char_ = utility::is_break_char(atom);
    is_integer_break_char_ = utility::is_integer_break_char(atom);
}

bool core::atom_status::is_dot() const
{
    return atom_ == '.';
}

bool core::atom_status::is_digit() const
{
    return is_digit_;
}

bool core::atom_status::is_whitespace() const
{
    return is_whitespace_;
}

bool core::atom_status::is_break_char() const
{
    return is_break_char_;
}

bool core::atom_status::is_integer_break_char() const
{
    return is_integer_break_char_;
}

bool core::atom_status::is_alpha() const
{
    return is_alpha_;
}

bool core::atom_status::is_alpha_digit() const
{
    return is_alpha_digit_;
}

bool core::atom_status::is_alpha_digit_underscore() const
{
    return is_alpha_digit_underscore_;
}

bool core::atom_status::is_empty() const
{
    return atom_ == L'\x0';
}

bool core::atom_status::is_empty_or_whitespace() const
{
    return is_empty() || is_whitespace();
}

bool core::atom_status::breaks_any() const
{
    return is_empty() || is_whitespace() || is_break_char();
}

bool core::atom_status::breaks_any_integer() const
{
    return is_empty() || is_whitespace() || is_integer_break_char();
}

bool core::atom_status::breaks_any_string() const
{
    return atom_ == '"';
}

bool core::atom_status::is_underscore() const
{
    return atom_ == '_';
}

bool core::atom_status::is_identifier_char() const
{
    return is_alpha_digit_underscore();
}

wchar_t core::atom_status::get_atom() const
{
    return atom_;
}
