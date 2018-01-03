#include "scanner.h"

core::scan_state core::scanner::get_initial_state(const std::wstring& input)
{
    return scan_state(std::wstring(input));
}

core::token core::scanner::scan_one(scan_state& state, token_type type)
{
    bool failed = false;
    token tok(token_type::INVALID);

    // throw on buffer end
    // TODO: make this nicer
    state.get_char();

    switch (type) {
    case INTEGER_LITERAL:
        tok = state.scan_integer_literal();
        break;
    case STRING_LITERAL:
        tok = state.scan_string_literal();
        break;
    default:
        throw exceptions::scan_failure(L"blah blah failure...");
    }

    return tok;
}
