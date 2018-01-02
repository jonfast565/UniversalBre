#include "stdafx.h"
#include "scanner.h"

ref_scan_state core::scanner::get_initial_state(const std::wstring& input)
{
    auto state = std::unique_ptr<scan_state>(
        new scan_state(std::wstring(input)));
    return std::move(state);
}
