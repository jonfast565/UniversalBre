#include "stdafx.h"
#include "scan_state.h"

void core::scan_state::increment_location(int increment)
{
    this->_location += increment;
    this->_column += increment;
}

core::scan_state::scan_state(std::wstring input)
{
    this->_input = input;
}

core::scan_state::scan_state(const scan_state & state)
{
    this->_column = state._column;
    this->_line = state._line;
    this->_location = state._location;
    this->_input = state._input;
}

wchar_t core::scan_state::get_char()
{
    if (this->_location > this->_input.size() || this->_location < 0) {
        throw exceptions::argument_out_of_range(this->_location);
    }
    return this->_input[this->_location];
}

void core::scan_state::scan_integer()
{

}

core::scan_state::~scan_state()
{
}
