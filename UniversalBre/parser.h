#pragma once

#include "scanner.h"
#include "expression_node.h"
#include "binop_expression_node.h"
#include "literal_expression_node.h"
#include "singleop_expression_node.h"
#include "op_types.h"

namespace core {
    class parser
    {
    private:
        int _location = 0;
        token_vecptr_s _tokens;
        log_ptr_s _log_object;

        token_type lookahead();
        token get_token();
        void eat_token(token_type actual, token_type expected);

        expression_node_ptr_s parse_program();
        expression_node_ptr_s parse_expression();
        expression_node_ptr_s parse_math_expression();
        expression_node_ptr_s parse_term();
        expression_node_ptr_s parse_factor();
    public:
        parser(token_vecptr_s tokens, log_ptr_s log_object) : 
            _tokens(tokens), _log_object(log_object) {}
        virtual ~parser() { }
        expression_node_ptr_s parse();
        void reset() { _location = 0; }
    };
    PTR_ALIAS(parser)
}

