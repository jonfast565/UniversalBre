#include "scan_state.h"
#include "atom_status.h"

void core::scan_state::increment_location(int increment)
{
    this->_location += increment;
    this->_column += increment;
    this->_input_left = _input.substr(_location, _input.length());
}

core::scan_state::scan_state(std::wstring input)
{
    this->_input = input;
    this->_input_left = std::wstring(input);
}

core::scan_state::scan_state(const scan_state & state)
{
    this->_column = state._column;
    this->_line = state._line;
    this->_location = state._location;
    this->_input = state._input;
    this->_input_left = state._input_left;
}

wchar_t core::scan_state::get_char()
{
    if (this->_location >= this->_input.size() || this->_location < 0) {
        throw exceptions::argument_out_of_range(this->_location);
    }
    return this->_input[this->_location];
}

wchar_t core::scan_state::get_char(int offset)
{
    if (this->_location + offset >= this->_input.size() 
        || this->_location + offset < 0) {
        throw exceptions::argument_out_of_range(this->_location + offset);
    }
    return this->_input[this->_location + offset];
}

void core::scan_state::skip_whitespace()
{
    int temp_ctr = 0;
    wchar_t c = get_char(temp_ctr);
    while (utility::is_whitespace(c)) {
        c = get_char(temp_ctr);
        temp_ctr++;
    }
    increment_location(temp_ctr);
}

core::token core::scan_state::scan_integer_literal()
{
    std::wstring result;
    while (true) {
        auto atom_status = core::atom_status(get_char());
        if (!atom_status.is_digit() && !atom_status.is_whitespace()
            || atom_status.is_digit() && atom_status.is_empty()) {
            throw exceptions::scan_failure(get_char(), L"integer");
        }
        if (atom_status.is_digit() && !atom_status.is_whitespace()) {
            result += get_char();
            increment_location(1);
        }
        if (atom_status.is_whitespace()) {
            break;
        }
    }
    return token(token_type::INTEGER_LITERAL, result);
}

core::token core::scan_state::scan_string_literal()
{
    return core::token(token_type::STRING_LITERAL);
}

core::token core::scan_state::scan_identifier()
{
    return core::token(token_type::IDENTIFIER);
}

core::token core::scan_state::scan_plus_operator()
{
    return core::token(token_type::PLUS_OPERATOR);
}

core::token core::scan_state::scan_minus_operator()
{
    return core::token(token_type::MINUS_OPERATOR);
}

core::token core::scan_state::scan_multiply_operator()
{
    return core::token(token_type::MULTIPLY_OPERATOR);
}

core::token core::scan_state::scan_divide_operator()
{
    return core::token(token_type::DIVIDE_OPERATOR);
}

core::token core::scan_state::scan_float_literal()
{
    return core::token(token_type::FLOAT_LITERAL);
}

core::scan_state::~scan_state()
{
}
