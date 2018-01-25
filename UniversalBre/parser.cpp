#include "parser.h"
#include "op_types.h"
#include "binop_expression_node.h"

core::expression_node_ptr_s core::parser::parse()
{
    log_object_->log_debug(L"Parse program");
    // TODO: This is wrong
    parse_program();
    // TODO: This is worse than above
    return nullptr;
}

void core::parser::reset()
{
    location_ = 0;
}

core::token_type core::parser::lookahead() const
{
    return tokens_->at(location_).get_type();
}

core::token core::parser::get_token() const
{
    return tokens_->at(location_);
}

void core::parser::eat_token(
    const token_type actual,
    const token_type expected)
{
    auto actual_str_type = get_token_type_string(actual);
    auto expected_str_type = get_token_type_string(expected);

    if (actual == expected)
        log_object_->log_success(L"Matched " + actual_str_type + L" with " + expected_str_type);

    if (actual != expected)
    {
        throw exceptions::parse_failure(
            utility::wstring_to_wcstr(actual_str_type),
            utility::wstring_to_wcstr(expected_str_type));
    }
    location_++;
}

core::statement_node_vecptrptr_s core::parser::parse_program()
{
    log_object_->log_debug(L"Parse program");

    auto statements =
        utility::make_ptr_s(statement_node_ptr_vec_s());

    while (lookahead() != token_type::end_of_file)
    {
        statements->push_back(parse_possible_statement());
    }

    eat_token(lookahead(), token_type::end_of_file);
    return statements;
}

core::statement_node_ptr_s core::parser::parse_possible_statement()
{
    switch (lookahead()) {
    case token_type::identifier:
    {
        const auto result = parse_assignment_statement();
        return result;
    }
    case token_type::infinite_keyword:
    {
        return nullptr;
    }
    default:
        throw exceptions::parse_failure(L"Blah"); // TODO: FIX
    }
}

void core::parser::print_expression(expression_node_ptr_s& expression) const
{
    log_object_->log_debug(L"AST side-view");
    expression->print(0);
}

core::assignment_node_ptr_s core::parser::parse_assignment_statement()
{
    log_object_->log_debug(L"Parse function/var assignment statement");

    const auto id_name = get_token().get_lexeme();
    eat_token(lookahead(), token_type::identifier);
    eat_token(lookahead(), token_type::assignment_operator);

    switch (lookahead())
    {
    case token_type::function_keyword:
    {
        const auto function_expression = parse_function_expression();
        auto result = utility::make_ptr_s(assignment_node(id_name, function_expression));
        eat_token(lookahead(), token_type::semicolon);
        return result;
    }
    default:
    {
        auto expression = parse_expression();
        print_expression(expression);
        auto result = utility::make_ptr_s(assignment_node(id_name, expression));
        eat_token(lookahead(), token_type::semicolon);
        return result;
    }
    }
}

core::function_expression_node_ptr_s core::parser::parse_function_expression()
{
    auto result_expression = utility::make_ptr_s(function_expression_node());

    eat_token(lookahead(), token_type::function_keyword);
    if (lookahead() == token_type::left_parenthesis)
    {
        auto argument_list = parse_argument_list();
        result_expression->get_argument_list();
    }

    eat_token(lookahead(), token_type::scope_begin_operator);
    while (lookahead() != token_type::scope_end_operator)
    {
        auto assignment = parse_possible_statement();
        // TODO: Add to function expression node
    }

    eat_token(lookahead(), token_type::scope_end_operator);
    return result_expression;
}

core::argument_list_node_ptr_s core::parser::parse_argument_list()
{
    eat_token(lookahead(), token_type::left_parenthesis);
    auto argument_list = utility::make_ptr_s(argument_list_node());
    while (lookahead() == token_type::identifier)
    {
        const auto cur_token = utility::make_ptr_s(get_token());
        eat_token(lookahead(), token_type::identifier);
        argument_list->add_argument(cur_token);
        if (lookahead() != token_type::right_parenthesis)
        {
            eat_token(lookahead(), token_type::list_delimiter);
        }
    }
    eat_token(lookahead(), token_type::right_parenthesis);
    return argument_list;
}

core::expression_node_ptr_s core::parser::parse_expression()
{
    log_object_->log_debug(L"Parse expression");
    auto result = parse_boolean_or_expression();
    return result;
}

core::expression_node_ptr_s core::parser::parse_boolean_or_expression()
{
    log_object_->log_debug(L"Parse boolean or expression");
    auto left_node = parse_boolean_and_expression();
    while (lookahead() == token_type::boolean_or_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::boolean_or_operator:
            eat_token(lookahead(), token_type::boolean_or_operator);
            type = op_type::op_boolean_or;
            break;
        default: break;
        }
        const auto right_node = parse_boolean_and_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_boolean_and_expression()
{
    log_object_->log_debug(L"Parse boolean and expression");
    auto left_node = parse_boolean_comparison_expression();
    while (lookahead() == token_type::boolean_and_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::boolean_and_operator:
            eat_token(lookahead(), token_type::boolean_and_operator);
            type = op_type::op_boolean_and;
            break;
        default: break;
        }
        const auto right_node = parse_boolean_comparison_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_boolean_comparison_expression()
{
    log_object_->log_debug(L"Parse boolean comparison expression");
    auto left_node = parse_boolean_equality_expression();
    while (lookahead() == token_type::boolean_gt_operator
        || lookahead() == token_type::boolean_lt_operator
        || lookahead() == token_type::boolean_gte_operator
        || lookahead() == token_type::boolean_lte_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::boolean_gt_operator:
            eat_token(lookahead(), token_type::boolean_gt_operator);
            type = op_type::op_boolean_gt;
            break;
        case token_type::boolean_lt_operator:
            eat_token(lookahead(), token_type::boolean_lt_operator);
            type = op_type::op_boolean_lt;
            break;
        case token_type::boolean_lte_operator:
            eat_token(lookahead(), token_type::boolean_lte_operator);
            type = op_type::op_boolean_lte;
            break;
        case token_type::boolean_gte_operator:
            eat_token(lookahead(), token_type::boolean_gte_operator);
            type = op_type::op_boolean_gte;
            break;
        default: break;
        }
        const auto right_node = parse_boolean_equality_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_boolean_equality_expression()
{
    log_object_->log_debug(L"Parse boolean equality expression");
    auto left_node = parse_concat_expression();
    while (lookahead() == token_type::boolean_eq_operator
        || lookahead() == token_type::boolean_ne_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::boolean_eq_operator:
            eat_token(lookahead(), token_type::boolean_eq_operator);
            type = op_type::op_boolean_eq;
            break;
        case token_type::boolean_ne_operator:
            eat_token(lookahead(), token_type::boolean_ne_operator);
            type = op_type::op_boolean_ne;
            break;
        default: break;
        }
        const auto right_node = parse_concat_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_concat_expression()
{
    log_object_->log_debug(L"Parse concat expression");
    auto left_node = parse_math_expression();
    while (lookahead() == token_type::concat_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::concat_operator:
            eat_token(lookahead(), token_type::concat_operator);
            type = op_type::op_concat_strings;
            break;
        default: break;
        }
        const auto right_node = parse_math_expression();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_math_expression()
{
    log_object_->log_debug(L"Parse mathematical expression");
    auto left_node = parse_term();
    while (lookahead() == token_type::plus_operator
        || lookahead() == token_type::minus_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::plus_operator:
            eat_token(lookahead(), token_type::plus_operator);
            type = op_type::op_addition;
            break;
        case token_type::minus_operator:
            type = op_type::op_subtraction;
            eat_token(lookahead(), token_type::minus_operator);
            break;
        default: break;
        }
        const auto right_node = parse_term();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_term()
{
    log_object_->log_debug(L"Parse term expression");
    auto left_node = parse_factor();
    while (lookahead() == token_type::multiply_operator
        || lookahead() == token_type::divide_operator)
    {
        auto type = op_type::op_invalid;
        switch (lookahead())
        {
        case token_type::multiply_operator:
            eat_token(lookahead(), token_type::multiply_operator);
            type = op_type::op_multiplication;
            break;
        case token_type::divide_operator:
            type = op_type::op_division;
            eat_token(lookahead(), token_type::divide_operator);
            break;
        default: break;
        }
        const auto right_node = parse_factor();
        left_node = utility::make_ptr_s(
            binop_expression_node(left_node, right_node, type));
    }
    return left_node;
}

core::expression_node_ptr_s core::parser::parse_factor()
{
    log_object_->log_debug(L"Parse factor/sub-expression");
    switch (lookahead())
    {
    case token_type::left_parenthesis:
    {
        eat_token(lookahead(), token_type::left_parenthesis);
        auto single = parse_expression();
        eat_token(lookahead(), token_type::right_parenthesis);
        return single;
    }
    case token_type::integer_literal:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::integer_literal);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    case token_type::float_literal:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::float_literal);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    case token_type::identifier:
    {
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::identifier);
        // TODO: Allow parsing of function calls
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    case token_type::string_literal:
    {
        // The grammar can allow this kind of interleaved expression
        // however, semantic analysis should provide errors on invalid operations
        // even though concatenation makes sense in this case
        auto cur_lexeme = get_token().get_lexeme();
        eat_token(lookahead(), token_type::string_literal);
        return utility::make_ptr_s(literal_expression_node(cur_lexeme));
    }
    case token_type::minus_operator:
    {
        eat_token(lookahead(), token_type::minus_operator);

        // TODO: Fix this shit, we're appending contextual information in the parser
        // as if it is scanning... dreadful.
        auto cur_lexeme = L"-" + get_token().get_lexeme();

        switch (lookahead())
        {
        case token_type::integer_literal:
            eat_token(lookahead(), token_type::integer_literal);
            return utility::make_ptr_s(literal_expression_node(cur_lexeme));
            break;
        case token_type::float_literal:
            eat_token(lookahead(), token_type::float_literal);
            return utility::make_ptr_s(literal_expression_node(cur_lexeme));
            break;
        case token_type::identifier:
            eat_token(lookahead(), token_type::identifier);
            return utility::make_ptr_s(literal_expression_node(cur_lexeme));
            break;
        default:
            auto error_str = L"Negative subexpression not allowed for " + cur_lexeme;
            throw exceptions::parse_failure(error_str.c_str());
        }
    }
    default:
        throw exceptions::parse_failure(
            L"Subexpression did not start with id, left parenthesis, identifier, or numeric constant");
    }
}

core::parser::parser(token_vecptr_s tokens, log_ptr_s log_object) :
    tokens_(std::move(tokens)),
    log_object_(std::move(log_object))
{
}
