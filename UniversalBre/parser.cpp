#include "parser.h"

core::expression_node_ptr_s core::parser::parse()
{
    _log_object->log_debug(L"Parse program");
    // TODO: This is wrong
    parse_program();
    // TODO: This is worse than above
    return nullptr;
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

core::assignment_node_vecptrptr_s core::parser::parse_program()
{
    _log_object->log_debug(L"Parse program");

    assignment_node_vecptrptr_s assignment_statements =
        utility::make_ptr_s(std::vector<assignment_node_ptr_s>());

    while (lookahead() != token_type::END_OF_FILE) {
        auto result = parse_assignment_statement();
        assignment_statements->push_back(result);
    }

    eat_token(lookahead(), token_type::END_OF_FILE);
    return assignment_statements;
}

void core::parser::print_expression(core::expression_node_ptr_s &expression)
{
    _log_object->log_debug(L"AST side-view");
    expression->print(0);
}

core::assignment_node_ptr_s core::parser::parse_assignment_statement()
{
    _log_object->log_debug(L"Parse function/var assignment statement");

    auto id_name = get_token().get_lexeme();
    eat_token(lookahead(), token_type::IDENTIFIER);
    eat_token(lookahead(), token_type::ASSIGNMENT_OPERATOR);

    switch (lookahead()) {
    case token_type::FUNCTION_KEYWORD:
    {
        auto function_expression = parse_function_expression();
        auto result = utility::make_ptr_s(assignment_node(id_name, function_expression));
        eat_token(lookahead(), token_type::SEMICOLON);
        return result;
    }
    default:
    {
        auto expression = parse_expression();
        print_expression(expression);
        auto result = utility::make_ptr_s(assignment_node(id_name, expression));
        eat_token(lookahead(), token_type::SEMICOLON);
        return result;
    }
    break;
    }
}

core::function_expression_node_ptr_s core::parser::parse_function_expression()
{
    auto result_expression = utility::make_ptr_s(function_expression_node());
    
    eat_token(lookahead(), token_type::FUNCTION_KEYWORD);
    if (lookahead() == token_type::LEFT_PARENTHESIS) {
        auto argument_list = parse_argument_list();
        result_expression->get_argument_list();
    }

    eat_token(lookahead(), token_type::SCOPE_BEGIN_OPERATOR);
    while (lookahead() != token_type::SCOPE_END_OPERATOR) {
        auto assignment = parse_assignment_statement();
        // TODO: Add to function expression node
    }

    eat_token(lookahead(), token_type::SCOPE_END_OPERATOR);
    return result_expression;
}

core::argument_list_node_ptr_s core::parser::parse_argument_list()
{
    eat_token(lookahead(), token_type::LEFT_PARENTHESIS);
    auto argument_list = utility::make_ptr_s(argument_list_node());
    while (lookahead() == token_type::IDENTIFIER) {
        auto cur_token = utility::make_ptr_s(get_token());
        eat_token(lookahead(), token_type::IDENTIFIER);
        argument_list->add_argument(cur_token);
        if (lookahead() != token_type::RIGHT_PARENTHESIS) {
            eat_token(lookahead(), token_type::LIST_DELIMITER);
        }
    }
    eat_token(lookahead(), token_type::RIGHT_PARENTHESIS);
    return argument_list;
}

core::expression_node_ptr_s core::parser::parse_expression()
{
    _log_object->log_debug(L"Parse expression");
    auto result = parse_boolean_or_expression();
    return result;
}

core::expression_node_ptr_s core::parser::parse_boolean_or_expression()
{
    _log_object->log_debug(L"Parse boolean or expression");
    auto left_node = parse_boolean_and_expression();
    while (lookahead() == token_type::BOOLEAN_OR_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::BOOLEAN_OR_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_OR_OPERATOR);
            type = op_type::OP_BOOLEAN_OR;
            break;
        }
        auto right_node = parse_boolean_and_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_boolean_and_expression()
{
    _log_object->log_debug(L"Parse boolean and expression");
    auto left_node = parse_boolean_comparison_expression();
    while (lookahead() == token_type::BOOLEAN_AND_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::BOOLEAN_AND_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_AND_OPERATOR);
            type = op_type::OP_BOOLEAN_AND;
            break;
        }
        auto right_node = parse_boolean_comparison_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_boolean_comparison_expression()
{
    _log_object->log_debug(L"Parse boolean comparison expression");
    auto left_node = parse_boolean_equality_expression();
    while (lookahead() == token_type::BOOLEAN_GT_OPERATOR
        || lookahead() == token_type::BOOLEAN_LT_OPERATOR
        || lookahead() == token_type::BOOLEAN_GTE_OPERATOR
        || lookahead() == token_type::BOOLEAN_LTE_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::BOOLEAN_GT_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_GT_OPERATOR);
            type = op_type::OP_BOOLEAN_GT;
            break;
        case token_type::BOOLEAN_LT_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_LT_OPERATOR);
            type = op_type::OP_BOOLEAN_LT;
            break;
        case token_type::BOOLEAN_LTE_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_LTE_OPERATOR);
            type = op_type::OP_BOOLEAN_LTE;
            break;
        case token_type::BOOLEAN_GTE_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_GTE_OPERATOR);
            type = op_type::OP_BOOLEAN_GTE;
            break;
        }
        auto right_node = parse_boolean_equality_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_boolean_equality_expression()
{
    _log_object->log_debug(L"Parse boolean equality expression");
    auto left_node = parse_concat_expression();
    while (lookahead() == token_type::BOOLEAN_EQ_OPERATOR
        || lookahead() == token_type::BOOLEAN_NE_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::BOOLEAN_EQ_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_EQ_OPERATOR);
            type = op_type::OP_BOOLEAN_EQ;
            break;
        case token_type::BOOLEAN_NE_OPERATOR:
            eat_token(lookahead(), token_type::BOOLEAN_NE_OPERATOR);
            type = op_type::OP_BOOLEAN_NE;
            break;
        }
        auto right_node = parse_concat_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_concat_expression()
{
    _log_object->log_debug(L"Parse concat expression");
    auto left_node = parse_math_expression();
    while (lookahead() == token_type::CONCAT_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::CONCAT_OPERATOR:
            eat_token(lookahead(), token_type::CONCAT_OPERATOR);
            type = op_type::OP_CONCAT_STRINGS;
            break;
        }
        auto right_node = parse_math_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_math_expression()
{
    _log_object->log_debug(L"Parse mathematical expression");
    auto left_node = parse_term();
    while (lookahead() == token_type::PLUS_OPERATOR
        || lookahead() == token_type::MINUS_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::PLUS_OPERATOR:
            eat_token(lookahead(), token_type::PLUS_OPERATOR);
            type = op_type::OP_ADDITION;
            break;
        case token_type::MINUS_OPERATOR:
            type = op_type::OP_SUBTRACTION;
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
    _log_object->log_debug(L"Parse term expression");
    auto left_node = parse_factor();
    while (lookahead() == token_type::MULTIPLY_OPERATOR
        || lookahead() == token_type::DIVIDE_OPERATOR) {
        op_type type = op_type::OP_INVALID;
        switch (lookahead()) {
        case token_type::MULTIPLY_OPERATOR:
            eat_token(lookahead(), token_type::MULTIPLY_OPERATOR);
            type = op_type::OP_MULTIPLICATION;
            break;
        case token_type::DIVIDE_OPERATOR:
            type = op_type::OP_DIVISION;
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
    _log_object->log_debug(L"Parse factor/sub-expression");
    switch (lookahead()) {
    case token_type::LEFT_PARENTHESIS:
    {
        eat_token(lookahead(), token_type::LEFT_PARENTHESIS);
        auto single = parse_expression();
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
    case token_type::FLOAT_LITERAL:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::FLOAT_LITERAL);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::IDENTIFIER:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::IDENTIFIER);
        // TODO: Allow parsing of function calls
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::STRING_LITERAL:
    {
        // The grammar can allow this kind of interleaved expression
        // however, semantic analysis should provide errors on invalid operations
        // even though concatenation makes sense in this case
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::STRING_LITERAL);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    break;
    case token_type::MINUS_OPERATOR:
    {
        eat_token(lookahead(), token_type::MINUS_OPERATOR);

        // TODO: Fix this shit, we're appending contextual information in the parser
        // as if it is scanning... dreadful.
        auto cur_lexeme = L"-" + get_token().get_lexeme();

        switch (lookahead()) {
        case token_type::INTEGER_LITERAL:
            eat_token(lookahead(), token_type::INTEGER_LITERAL);
            return utility::make_ptr_s(literal_expression_node(cur_lexeme));
            break;
        case token_type::FLOAT_LITERAL:
            eat_token(lookahead(), token_type::FLOAT_LITERAL);
            return utility::make_ptr_s(literal_expression_node(cur_lexeme));
            break;
        case token_type::IDENTIFIER:
            eat_token(lookahead(), token_type::IDENTIFIER);
            return utility::make_ptr_s(literal_expression_node(cur_lexeme));
            break;
        default:
            auto error_str = L"Negative subexpression not allowed for " + cur_lexeme;
            throw exceptions::parse_failure(error_str.c_str());
        }
    }
    break;
    default:
        throw exceptions::parse_failure(L"Subexpression did not start with id, left parenthesis, identifier, or numeric constant");
    }
}