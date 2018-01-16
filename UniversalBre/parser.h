#pragma once

#include "scanner.h"

namespace core {
    class parser
    {
    private:
        int _location = 0;
        token_vecptr_s _tokens;
        log_ptr_s _log_object;

        token_type get_cur_type();
        token_type get_next_type();
        void match_increment(token_type actual, token_type expected);

        void parse_program();
        void parse_expression();
        void parse_precedence_expression();
        void parse_addition_subtraction_expression();
        void parse_multiplication_division_expression();
        void parse_subexpression();
    public:
        parser(token_vecptr_s tokens, log_ptr_s log_object) : _tokens(tokens), _log_object(log_object) {}
        virtual ~parser() { }
        void parse();
        void reset() { _location = 0; }
    };
    PTR_ALIAS(parser)
}

