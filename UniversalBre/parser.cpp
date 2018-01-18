#include "parser.h"

core::expression_node_ptr_s core::parser::parse()
{
    _log_object->log_debug(L"Parse program");
    return parse_program();
}

core::token_type core::parser::lookahead()
{
    return _tokens->at(_location).get_type();
}

core::token core::parser::get_token()
{
    return _tokens->at(_location);
}

void core::parser::eat_token(
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
    _log_object->log_debug(L"Parse program");
    auto expression = parse_expression_internal();
    eat_token(lookahead(), token_type::END_OF_FILE);
    _log_object->log_debug(L"AST side-view");
    expression->print(0);
    return expression;
}

/*
expr   : term((PLUS | MINUS) term)*
    term : factor((MUL | DIV) factor)*
    factor : INTEGER | LPAREN expr RPAREN
*/

core::expression_node_ptr_s core::parser::parse_expression()
{
    _log_object->log_debug(L"Parse expression");
    auto result = parse_expression_internal();
    result->print(0);
    return result;
}

core::expression_node_ptr_s core::parser::parse_expression_internal()
{
    _log_object->log_debug(L"Parse term expression");
    auto left_node = parse_term();
    while (lookahead() == token_type::PLUS_OPERATOR
        || lookahead() == token_type::MINUS_OPERATOR) {
        binop_type type = OP_INVALID;
        switch (lookahead()) {
        case PLUS_OPERATOR:
            eat_token(lookahead(), token_type::PLUS_OPERATOR);
            type = OP_ADDITION;
            break;
        case MINUS_OPERATOR:
            type = OP_SUBTRACTION;
            eat_token(lookahead(), token_type::MINUS_OPERATOR);
            break;
        }
        auto right_node = parse_term();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_term()
{
    _log_object->log_debug(L"Parse factor expression");
    auto left_node = parse_factor();
    while (lookahead() == token_type::MULTIPLY_OPERATOR
        || lookahead() == token_type::DIVIDE_OPERATOR) {
        binop_type type = OP_INVALID;
        switch (lookahead()) {
        case MULTIPLY_OPERATOR:
            eat_token(lookahead(), token_type::MULTIPLY_OPERATOR);
            type = OP_MULTIPLICATION;
            break;
        case DIVIDE_OPERATOR:
            type = OP_DIVISION;
            eat_token(lookahead(), token_type::DIVIDE_OPERATOR);
            break;
        }
        auto right_node = parse_factor();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_factor()
{
    _log_object->log_debug(L"Parse sub-expression");
    switch (lookahead()) {
    case token_type::LEFT_PARENTHESIS:
    {
        eat_token(lookahead(), token_type::LEFT_PARENTHESIS);
        auto single = parse_expression_internal();
        eat_token(lookahead(), token_type::RIGHT_PARENTHESIS);
        return single;
    }
    break;
    case token_type::INTEGER_LITERAL:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::INTEGER_LITERAL);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::IDENTIFIER:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::IDENTIFIER);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::MINUS_OPERATOR:
    {
        eat_token(lookahead(), token_type::MINUS_OPERATOR);
        auto cur_lexeme = L"-" + get_token().get_lexeme();
        eat_token(lookahead(), token_type::INTEGER_LITERAL);
        // TODO: Fix this shit, we're appending contextual information in the parser
        // as if it is scanning... dreadful.
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    default:
        throw exceptions::parse_failure(L"Subexpression did not start with id, left parenthesis, or identifier");
    }
}