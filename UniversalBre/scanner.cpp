#include "stdafx.h"
#include "scanner.h"

core::scanner::scanner(const std::wstring& input)
{
    this->initial_state = std::unique_ptr<scan_state>(
        new scan_state(std::wstring(input)));
}

core::scanner::~scanner()
{
}

std::unique_ptr<core::scan_state> core::scanner::get_initial_state()
{
    auto state = std::unique_ptr<scan_state>(
        new scan_state(*this->initial_state));
    return std::move(state);
}
