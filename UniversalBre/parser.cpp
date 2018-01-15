#include "parser.h"

void core::parser::parse()
{
    _log_object->log_debug(L"Parse program");
    parse_program();
}

core::token_type core::parser::get_cur_type()
{
    return _tokens->at(_location).get_type();
}

core::token_type core::parser::get_next_type()
{
    // TODO: Naive, could cause major error 
    // if bounds aren't properly checked
    return _tokens->at(_location + 1).get_type();
}

void core::parser::match_increment(
    token_type actual, 
    token_type expected)
{
    auto actual_str_type = get_token_type_string(actual);
    auto expected_str_type = get_token_type_string(expected);

    if (actual == expected)
        _log_object->log_success(L"Matched " + actual_str_type + L" with " + expected_str_type);

    if (actual != expected) {
        throw exceptions::parse_failure(
            utility::wstring_to_wcstr(actual_str_type),
            utility::wstring_to_wcstr(expected_str_type));
    }
    _location++;
}

void core::parser::parse_program()
{
    _log_object->log_debug(L"Parse expression");
    parse_expression();
}

void core::parser::parse_expression()
{
    switch (get_cur_type()) {
    case token_type::LEFT_PARENTHESIS:
        match_increment(get_cur_type(), token_type::LEFT_PARENTHESIS);
        parse_single_expression();
        match_increment(get_cur_type(), token_type::RIGHT_PARENTHESIS);
        break;
    default:
        parse_single_expression();
    }
    match_increment(get_cur_type(), token_type::END_OF_FILE);
}

void core::parser::parse_single_expression()
{
    _log_object->log_debug(L"Parse single expression");
    match_increment(get_cur_type(), token_type::INTEGER_LITERAL);
    switch (get_cur_type()) {
    case token_type::PLUS_OPERATOR:
        match_increment(get_cur_type(), token_type::PLUS_OPERATOR);
        break;
    case token_type::MINUS_OPERATOR:
        match_increment(get_cur_type(), token_type::MINUS_OPERATOR);
        break;
    default:
        throw std::exception("Missing operand");
    }
    match_increment(get_cur_type(), token_type::INTEGER_LITERAL);
}

void core::parser::parse_addition_subtraction()
{

}

void core::parser::parse_multiplication_division()
{

}