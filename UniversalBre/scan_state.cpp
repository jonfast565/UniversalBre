#include "scan_state.h"
#include "atom_status.h"

void core::scan_state::increment_location(int increment)
{
    _location += increment;
    _column += increment;
    _input_left = _input.substr(_location, _input.length());
}

core::atom_status_ptr_s core::scan_state::get_char_atom()
{
    return utility::make_ptr_s(core::atom_status(get_char()));
}

core::scan_state::scan_state(std::wstring input)
{
    _input = input;
    _input_left = std::wstring(input);
}

core::scan_state::scan_state(const scan_state & state)
{
    _column = state._column;
    _line = state._line;
    _location = state._location;
    _input = state._input;
    _input_left = state._input_left;
}

wchar_t core::scan_state::get_char()
{
    if (_location >= _input.size() || _location < 0) {
        throw exceptions::argument_out_of_range(_location);
    }
    return _input[_location];
}

wchar_t core::scan_state::get_char(int offset)
{
    if (_location + offset >= _input.size() 
        || _location + offset < 0) {
        throw exceptions::argument_out_of_range(_location + offset);
    }
    return _input[_location + offset];
}

void core::scan_state::skip_whitespace()
{
    if (out_of_range()) return;

    int temp_ctr = 0;
    wchar_t c = get_char(temp_ctr);
    if (!utility::is_whitespace(c)) return;

    while (true) {
        c = get_char(temp_ctr);
        if (utility::is_whitespace(c))
            temp_ctr++;
        else
            break;
    }

    increment_location(temp_ctr);
}

core::token core::scan_state::try_scan_integer_literal()
{
    std::wstring result;

    auto first_char = core::atom_status(get_char());
    if (!first_char.is_digit()) {
        throw exceptions::scan_failure(get_char(), L"digit");
    }

    auto next_char = get_char_atom();
    do {
        if (next_char->is_digit()) {
            result += get_char();
            increment_location(1);
        }
        else {
            throw exceptions::scan_failure(get_char(), L"digit");
        }
        next_char = get_char_atom();
    } while (!next_char->breaks_any_integer());

    return token(token_type::INTEGER_LITERAL, result);
}

core::token core::scan_state::try_scan_string_literal()
{
    throw exceptions::not_implemented_exception(L"string lit scan");
    return core::token(token_type::STRING_LITERAL);
}

core::token core::scan_state::try_scan_float_literal()
{
    std::wstring result;

    auto first_char = core::atom_status(get_char());
    if (!first_char.is_digit()) {
        throw exceptions::scan_failure(get_char(), L"digit");
    }

    auto next_char = get_char_atom();
    bool precision_part = false;
    do {
        if (next_char->is_digit()) {
            result += get_char();
            increment_location(1);
        }
        else if (next_char->is_dot() && precision_part == false) {
            result += get_char();
            increment_location(1);
            precision_part = true;
        }
        else {
            throw exceptions::scan_failure(get_char(), L"digit");
        }
        next_char = get_char_atom();
    } while (!next_char->breaks_any_integer());

    return core::token(token_type::FLOAT_LITERAL, result);
}

core::token core::scan_state::try_scan_identifier()
{
    std::wstring result;

    auto first_char = core::atom_status(get_char());
    if (!first_char.is_identifier_char()) {
        throw exceptions::scan_failure(get_char(), L"letter, digit, underscore");
    }

    auto next_char = get_char_atom();
    do {
        if (next_char->is_identifier_char()) {
            result += get_char();
            increment_location(1);
        }
        else {
            throw exceptions::scan_failure(get_char(), L"letter, digit, underscore");
        }
        next_char = get_char_atom();
    } while (!next_char->breaks_any());
    return core::token(token_type::IDENTIFIER, result);
}

core::token core::scan_state::try_scan_plus_operator()
{
    if (get_char() == L'+') {
        increment_location(1);
        return core::token(token_type::PLUS_OPERATOR);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"plus operator");
    }
}

core::token core::scan_state::try_scan_minus_operator()
{
    if (get_char() == L'-') {
        increment_location(1);
        return core::token(token_type::MINUS_OPERATOR);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"minus operator");
    }
}

core::token core::scan_state::try_scan_multiply_operator()
{
    if (get_char() == L'*') {
        increment_location(1);
        return core::token(token_type::MULTIPLY_OPERATOR);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"multiply operator");
    }
}

core::token core::scan_state::try_scan_divide_operator()
{
    if (get_char() == L'/') {
        increment_location(1);
        return core::token(token_type::DIVIDE_OPERATOR);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"divide operator");
    }
}

core::token core::scan_state::try_scan_concat_operator()
{
    if (get_char() == L'~') {
        increment_location(1);
        return core::token(token_type::CONCAT_OPERATOR);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"concat operator");
    }
}

core::token core::scan_state::try_scan_left_parenthesis()
{
    if (get_char() == L'(') {
        increment_location(1);
        return core::token(token_type::LEFT_PARENTHESIS);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"left parenthesis");
    }
}

core::token core::scan_state::try_scan_right_parenthesis()
{
    if (get_char() == L')') {
        increment_location(1);
        return core::token(token_type::RIGHT_PARENTHESIS);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"right parenthesis");
    }
}

core::token core::scan_state::try_scan_end_of_file()
{
    if (out_of_range()) {
        return core::token(token_type::END_OF_FILE);
    }
    else {
        throw exceptions::scan_failure(get_char(), L"end of file");
    }
}

bool core::scan_state::out_of_range()
{
    return _location >= _input.size() - 1 || _location < 0;
}

core::scan_state::~scan_state()
{
}
