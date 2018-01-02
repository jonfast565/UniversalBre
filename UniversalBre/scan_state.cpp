#include "stdafx.h"
#include "scan_state.h"

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
    if (this->_location + offset >= this->_input.size() || this->_location + offset < 0) {
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

core::token core::scan_state::try_scan_integer()
{
    std::wstring result;
    while (true) {
        auto is_digit = utility::is_digit(get_char());
        auto is_whitespace = utility::is_whitespace(get_char());
        if (!is_digit && !is_whitespace 
            || is_whitespace && result.empty()) {
            throw exceptions::scan_failure("Expected integer, got " + get_char());
        }
        if (is_digit && !is_whitespace) {
            result += get_char();
            increment_location(1);
        }
        if (is_whitespace) {
            break;
        }
    }
    return token(token_type::INTEGER_LITERAL, result);
}

core::scan_state::~scan_state()
{
}
