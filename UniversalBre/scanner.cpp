#include "stdafx.h"
#include "scanner.h"

core::scan_state core::scanner::get_initial_state(const std::wstring& input)
{
    return scan_state(std::wstring(input));
}

core::token core::scanner::scan_one(scan_state& state)
{
    return token(token_type::END_OF_FILE);
}

void core::scanner::scan_all(const std::wstring& input)
{
    auto state = get_initial_state(input);
    token current_token(token_type::BEGIN_OF_FILE);
    while (current_token.get_type() != token_type::END_OF_FILE) {
        current_token = scan_one(state);
    }
}
