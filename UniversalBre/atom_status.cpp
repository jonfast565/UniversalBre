#include "atom_status.h"

core::atom_status::atom_status(const wchar_t atom) : _atom(atom)
{
    _is_digit = utility::is_digit(atom);
    _is_whitespace = utility::is_whitespace(atom);
    _is_alpha = utility::is_alpha(atom);
    _is_alpha_digit = utility::is_alpha_digit(atom);
    _is_alpha_digit_underscore = false; // TODO: FIX
}

bool core::atom_status::is_digit()
{
    return _is_digit;
}

bool core::atom_status::is_whitespace()
{
    return _is_whitespace;
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
