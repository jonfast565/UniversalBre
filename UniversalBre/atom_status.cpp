#include "atom_status.h"

core::atom_status::atom_status(const wchar_t atom) : _atom(atom)
{
    _is_digit = utility::is_digit(atom);
    _is_whitespace = utility::is_whitespace(atom);
    _is_alpha = utility::is_alpha(atom);
    _is_alpha_digit = utility::is_alpha_digit(atom);
    _is_alpha_digit_underscore = utility::is_alpha_digit(atom) || is_underscore();
    _is_break_char = utility::is_break_char(atom);
    _is_integer_break_char = utility::is_integer_break_char(atom);
}

bool core::atom_status::is_dot()
{
    return _atom == '.';
}

bool core::atom_status::is_digit()
{
    return _is_digit;
}

bool core::atom_status::is_whitespace()
{
    return _is_whitespace;
}

bool core::atom_status::is_break_char()
{
    return _is_break_char;
}

bool core::atom_status::is_integer_break_char()
{
    return _is_integer_break_char;
}

bool core::atom_status::is_alpha()
{
    return _is_alpha;
}

bool core::atom_status::is_alpha_digit()
{
    return _is_alpha_digit;
}

bool core::atom_status::is_alpha_digit_underscore()
{
    return _is_alpha_digit_underscore;
}

bool core::atom_status::is_empty()
{
    return _atom == L'\x0';
}

bool core::atom_status::is_empty_or_whitespace()
{
    return is_empty() || is_whitespace();
}

bool core::atom_status::breaks_any()
{
    return is_empty() || is_whitespace() || is_break_char();
}

bool core::atom_status::breaks_any_integer()
{
    return is_empty() || is_whitespace() || is_integer_break_char();
}

bool core::atom_status::breaks_any_string()
{
    return _atom == '"';
}

bool core::atom_status::is_underscore()
{
    return _atom == '_';
}

bool core::atom_status::is_identifier_char()
{
    return is_alpha_digit_underscore();
}

wchar_t core::atom_status::get_atom()
{
    return _atom;
}
