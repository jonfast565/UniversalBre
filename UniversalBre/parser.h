#pragma once

#include "scanner.h"
#include "expression_node.h"
#include "binop_expression_node.h"
#include "literal_expression_node.h"
#include "singleop_expression_node.h"
#include "binop_types.h"
#include "expression_pruner.h"

namespace core {
    class parser
    {
    private:
        int _location = 0;
        token_vecptr_s _tokens;
        log_ptr_s _log_object;

        token_type get_cur_type();
        token_type get_next_type();
        std::wstring get_cur_lexeme();
        void match_increment(token_type actual, token_type expected);

        expression_node_ptr_s parse_program();
        expression_node_ptr_s parse_expression();
        expression_node_ptr_s parse_precedence_expression();
        expression_node_ptr_s parse_addition_subtraction_expression();
        expression_node_ptr_s parse_multiplication_division_expression();
        expression_node_ptr_s parse_subexpression();
    public:
        parser(token_vecptr_s tokens, log_ptr_s log_object) : _tokens(tokens), _log_object(log_object) {}
        virtual ~parser() { }
        expression_node_ptr_s parse();
        void reset() { _location = 0; }
    };
    PTR_ALIAS(parser)
}

