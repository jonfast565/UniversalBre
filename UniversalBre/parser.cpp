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
    parse_expression();
    match_increment(get_cur_type(), token_type::END_OF_FILE);
}

void core::parser::parse_expression()
{
    _log_object->log_debug(L"Parse expression");
    parse_precedence_expression();
    parse_addition_subtraction_expression();
}

void core::parser::parse_precedence_expression()
{
    parse_subexpression();
    parse_multiplication_division_expression();
}

void core::parser::parse_addition_subtraction_expression()
{
    _log_object->log_debug(L"Parse addition/subtraction expression");
    switch (get_cur_type()) {
    case token_type::PLUS_OPERATOR:
        match_increment(get_cur_type(), token_type::PLUS_OPERATOR);
        parse_precedence_expression();
        parse_addition_subtraction_expression();
        break;
    case token_type::MINUS_OPERATOR:
        match_increment(get_cur_type(), token_type::MINUS_OPERATOR);
        parse_precedence_expression();
        parse_addition_subtraction_expression();
        break;
    }
}

void core::parser::parse_multiplication_division_expression()
{
    _log_object->log_debug(L"Parse multiplication/division expression");
    switch (get_cur_type()) {
    case token_type::MULTIPLY_OPERATOR:
        match_increment(get_cur_type(), token_type::MULTIPLY_OPERATOR);
        parse_subexpression();
        parse_multiplication_division_expression();
        break;
    case token_type::DIVIDE_OPERATOR:
        match_increment(get_cur_type(), token_type::DIVIDE_OPERATOR);
        parse_subexpression();
        parse_multiplication_division_expression();
        break;
    }
}

void core::parser::parse_subexpression()
{
    _log_object->log_debug(L"Parse sub-expression");
    switch (get_cur_type()) {
    case token_type::LEFT_PARENTHESIS:
        match_increment(get_cur_type(), token_type::LEFT_PARENTHESIS);
        parse_expression();
        match_increment(get_cur_type(), token_type::RIGHT_PARENTHESIS);
        break;
    case token_type::INTEGER_LITERAL:
        match_increment(get_cur_type(), token_type::INTEGER_LITERAL);
        break;
    case token_type::IDENTIFIER:
        match_increment(get_cur_type(), token_type::IDENTIFIER);
        break;
    case token_type::MINUS_OPERATOR:
        match_increment(get_cur_type(), token_type::MINUS_OPERATOR);
        match_increment(get_cur_type(), token_type::INTEGER_LITERAL);
    default:
        _log_object->log_debug(L"Subexpression did not start with id, left parenthesis, or identifier");
    }
}