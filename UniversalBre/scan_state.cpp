#include "stdafx.h"
#include "scan_state.h"

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
    if (this->_location > this->_input.size()) {
        throw exceptions::argument_out_of_range(this->_location);
    }
    return this->_input[this->_location];
}

void core::scan_state::try_scan_integer()
{
    
}

void core::scan_state::increment_one()
{
    this->_location++;
    this->_column++;
}

void core::scan_state::increment_one_line()
{
    this->_location++;
    this->_column = 0;
    this->_line++;
}

core::scan_state::~scan_state()
{
}
