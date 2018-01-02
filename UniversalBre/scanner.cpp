#include "stdafx.h"
#include "scanner.h"

core::scan_state core::scanner::get_initial_state(const std::wstring& input)
{
    return scan_state(std::wstring(input));
}

core::token core::scanner::scan_one(scan_state& state)
{
    bool failed = false;
    token tok(token_type::INVALID);

    try {
        tok = state.try_scan_integer();
    }
    catch (exceptions::scan_failure& e) {
        failed = true;
    }

    if (failed) {
        throw exceptions::scan_failure("FATAL");
    }

    return tok;
}

std::vector<core::token> core::scanner::scan_all(const std::wstring& input)
{
    std::vector<core::token> token_list;
    auto state = get_initial_state(input);
    token current_token(token_type::BEGIN_OF_FILE);

    while (current_token.get_type() != token_type::END_OF_FILE) {
        try {
            state.skip_whitespace();
            current_token = scan_one(state);
            token_list.push_back(current_token);
        }
        catch (exceptions::argument_out_of_range&) {
            break;
        }
    }
    return token_list;
}
