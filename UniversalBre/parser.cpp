#include "parser.h"

core::expression_node_ptr_s core::parser::parse()
{
    _log_object->log_debug(L"Parse program");
    return parse_program();
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

std::wstring core::parser::get_cur_lexeme()
{
    return _tokens->at(_location).get_lexeme();
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

core::expression_node_ptr_s core::parser::parse_program()
{
    auto expression = parse_expression();
    expression->print(0);
    match_increment(get_cur_type(), token_type::END_OF_FILE);
    return expression;
}

core::expression_node_ptr_s core::parser::parse_expression()
{
    _log_object->log_debug(L"Parse expression");
    auto left = parse_precedence_expression();
    auto right = parse_addition_subtraction_expression();
    return utility::make_ptr_s(binop_expression_node(left, right, binop_type::OP_INVALID));
}

core::expression_node_ptr_s core::parser::parse_precedence_expression()
{
    auto left = parse_subexpression();
    auto right = parse_multiplication_division_expression();
    return utility::make_ptr_s(binop_expression_node(left, right, binop_type::OP_INVALID));
}

core::expression_node_ptr_s core::parser::parse_addition_subtraction_expression()
{
    _log_object->log_debug(L"Parse addition/subtraction expression");
    switch (get_cur_type()) {
    case token_type::PLUS_OPERATOR:
    {
        match_increment(get_cur_type(), token_type::PLUS_OPERATOR);
        auto left = parse_precedence_expression();
        auto right = parse_addition_subtraction_expression();
        return utility::make_ptr_s(binop_expression_node(left, right, binop_type::OP_ADDITION));
    }
    break;
    case token_type::MINUS_OPERATOR:
    {
        match_increment(get_cur_type(), token_type::MINUS_OPERATOR);
        auto left = parse_precedence_expression();
        auto right = parse_addition_subtraction_expression();
        return utility::make_ptr_s(binop_expression_node(left, right, binop_type::OP_SUBTRACTION));
    }
    break;
    default:
        return nullptr;
    }
}

core::expression_node_ptr_s core::parser::parse_multiplication_division_expression()
{
    _log_object->log_debug(L"Parse multiplication/division expression");
    switch (get_cur_type()) {
    case token_type::MULTIPLY_OPERATOR:
    {
        match_increment(get_cur_type(), token_type::MULTIPLY_OPERATOR);
        auto left = parse_subexpression();
        auto right = parse_multiplication_division_expression();
        return utility::make_ptr_s(binop_expression_node(left, right, binop_type::OP_MULTIPLICATION));
    }
    break;
    case token_type::DIVIDE_OPERATOR:
    {
        match_increment(get_cur_type(), token_type::DIVIDE_OPERATOR);
        auto left = parse_subexpression();
        auto right = parse_multiplication_division_expression();
        return utility::make_ptr_s(binop_expression_node(left, right, binop_type::OP_DIVISION));
    }
    break;
    default:
        return nullptr;
    }
}

core::expression_node_ptr_s core::parser::parse_subexpression()
{
    _log_object->log_debug(L"Parse sub-expression");
    switch (get_cur_type()) {
    case token_type::LEFT_PARENTHESIS:
    {
        match_increment(get_cur_type(), token_type::LEFT_PARENTHESIS);
        auto single = parse_expression();
        match_increment(get_cur_type(), token_type::RIGHT_PARENTHESIS);
        return utility::make_ptr_s(singleop_expression_node(single));
    }
    break;
    case token_type::INTEGER_LITERAL:
    {
        auto cur_lexeme = get_cur_lexeme();
        match_increment(get_cur_type(), token_type::INTEGER_LITERAL);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::IDENTIFIER:
    {
        auto cur_lexeme = get_cur_lexeme();
        match_increment(get_cur_type(), token_type::IDENTIFIER);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::MINUS_OPERATOR:
    {
        match_increment(get_cur_type(), token_type::MINUS_OPERATOR);
        auto cur_lexeme = L"-" + get_cur_lexeme();
        match_increment(get_cur_type(), token_type::INTEGER_LITERAL);
        // TODO: Fix this shit, we're appending contextual information in the parser
        // as if it is scanning... dreadful.
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    default:
        _log_object->log_debug(L"Subexpression did not start with id, left parenthesis, or identifier");
        return nullptr;
    }
}