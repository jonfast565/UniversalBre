#pragma once

#include "scanner.h"
#include "expression_node.h"
#include "assignment_node.h"
#include "argument_list_node.h"
#include "statement_node.h"

namespace core
{
    class parser
    {
        int location_ = 0;
        token_vecptr_s tokens_ = nullptr;
        log_ptr_s log_object_ = nullptr;

        token_type lookahead() const;
        token get_token() const;

        void eat_token(token_type actual, token_type expected);
        void print_expression(expression_node_ptr_s& expression) const;

        // program parsing
        statement_node_vecptrptr_s parse_program();
        statement_node_ptr_s parse_possible_statement();


        assignment_node_ptr_s parse_assignment_statement();
        function_expression_node_ptr_s parse_function_expression();
        argument_list_node_ptr_s parse_argument_list();

        // expression parsing
        expression_node_ptr_s parse_expression();
        expression_node_ptr_s parse_boolean_or_expression();
        expression_node_ptr_s parse_boolean_and_expression();
        expression_node_ptr_s parse_boolean_comparison_expression();
        expression_node_ptr_s parse_boolean_equality_expression();
        expression_node_ptr_s parse_concat_expression();
        expression_node_ptr_s parse_math_expression();
        expression_node_ptr_s parse_term();
        expression_node_ptr_s parse_factor();
    public:
        parser(token_vecptr_s tokens, log_ptr_s log_object);
        expression_node_ptr_s parse();
        void reset();
    };
    ALIAS_TYPES(parser)
}
