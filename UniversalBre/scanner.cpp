#include "scanner.h"

core::scan_state core::scanner::get_initial_state(const std::wstring& input)
{
    return scan_state(std::wstring(input));
}

core::scanner::scanner(core::log_ptr_s log_object) : _log_object(log_object)
{
}

core::scanner::~scanner()
{
}

core::token_vecptr_s core::scanner::scan_all(const std::wstring& input)
{
    auto token_list_ptr = utility::make_ptr_s(core::token_vec());
    auto start_state = utility::make_ptr_s(get_initial_state(input));
    token_ptr_s last_token = nullptr;

    auto pass_ctr = 1;
    _log_object->log_debug(L"Scan Pass #" + std::to_wstring(pass_ctr));

    while (last_token == nullptr || 
           last_token->get_type() != token_type::END_OF_FILE) {
        last_token = utility::make_ptr_s(scan_one(start_state));
        token_list_ptr->push_back(*last_token);
        pass_ctr++;
    }

    return token_list_ptr;
}

core::token core::scanner::scan_one(scan_state_ptr_s state)
{
    // skip all whitespace
    _log_object->log_debug(L"Skipping whitespace");
    state->skip_whitespace();

    try {
        auto t = state->try_scan_end_of_file();
        _log_object->log_success(L"EOF scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        _log_object->log_debug(L"EOF not scanned");
    }

    // save state to ensure we can rewind
    auto save_state = scan_state(*state);
    scan_state_ptr_s old_state = nullptr;

    // keywords... coming soon

    // literals
    try {
        auto t = state->try_scan_integer_literal();
        _log_object->log_success(L"Integer literal scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Integer literal not scanned");
    }

    try {
        auto t = state->try_scan_float_literal();
        _log_object->log_success(L"Float literal scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Float literal not scanned");
    }

    // parenthesis
    try {
        auto t = state->try_scan_left_parenthesis();
        _log_object->log_success(L"Left parenthesis scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        _log_object->log_debug(L"Left parenthesis not scanned");
    }

    try {
        auto t = state->try_scan_right_parenthesis();
        _log_object->log_success(L"Right parenthesis scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Right parenthesis not scanned");
    }

    // double char boolean operators
    try {
        auto t = state->try_scan_boolean_and_operator();
        _log_object->log_success(L"Boolean and operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean and operator not scanned");
    }

    try {
        auto t = state->try_scan_boolean_or_operator();
        _log_object->log_success(L"Boolean or operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean or operator not scanned");
    }

    try {
        auto t = state->try_scan_boolean_gte_operator();
        _log_object->log_success(L"Boolean or operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean greater-than-equal operator not scanned");
    }

    try {
        auto t = state->try_scan_boolean_lte_operator();
        _log_object->log_success(L"Boolean less-than-equal operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean less-than-equal operator not scanned");
    }

    try {
        auto t = state->try_scan_boolean_eq_operator();
        _log_object->log_success(L"Boolean equals operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean equals operator not scanned");
    }

    try {
        auto t = state->try_scan_boolean_ne_operator();
        _log_object->log_success(L"Boolean not-equals operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean not-equals operator not scanned");
    }

    // single char boolean operators
    try {
        auto t = state->try_scan_boolean_lt_operator();
        _log_object->log_success(L"Boolean less than operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean less than operator not scanned");
    }

    try {
        auto t = state->try_scan_boolean_gt_operator();
        _log_object->log_success(L"Boolean greater than operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Boolean greater than operator not scanned");
    }

    // single char operators for math
    try {
        auto t = state->try_scan_plus_operator();
        _log_object->log_success(L"Plus operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Plus operator not scanned");
    }

    try {
        auto t = state->try_scan_minus_operator();
        _log_object->log_success(L"Minus operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Minus operator not scanned");
    }

    try {
        auto t = state->try_scan_multiply_operator();
        _log_object->log_success(L"Multiply operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Multiply operator not scanned");
    }

    try {
        auto t = state->try_scan_divide_operator();
        _log_object->log_success(L"Divide operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Divide operator not scanned");
    }

    // special operators
    try {
        auto t = state->try_scan_concat_operator();
        _log_object->log_success(L"Concat operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Concat operator not scanned");
    }

    try {
        auto t = state->try_scan_semicolon();
        _log_object->log_success(L"Semicolon scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Semicolon not scanned");
    }

    try {
        auto t = state->try_scan_assignment_operator();
        _log_object->log_success(L"Assignment operator scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Assignment operator not scanned");
    }

    // ids
    try {
        auto t = state->try_scan_identifier();
        _log_object->log_success(L"Identifier scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"Identifier not scanned");
    }

    // string literals
    try {
        auto t = state->try_scan_string_literal();
        _log_object->log_success(L"String literal scanned");
        return t;
    }
    catch (exceptions::extended_exception&) {
        old_state = utility::make_ptr_s(scan_state(*state));
        *state = save_state;
        _log_object->log_debug(L"String literal not scanned");
    }

    // error case
    throw exceptions::scan_failure(state->get_char());
}
