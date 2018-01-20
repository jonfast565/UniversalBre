#pragma once

#include "global_defines.h"
#include "exceptions.h"
#include "utility.h"
#include "atom_status.h"
#include "token_types.h"
#include "token.h"

namespace core {
    class scan_state
    {
    private:
        // counters
        int _backtrack = 0;
        int _location = 0;
        int _line = 0;
        int _column = 0;

        // inputs
        std::wstring _input;
        std::wstring _input_left;

        // utilities
        void increment_location(int increment);
        atom_status_ptr_s get_char_atom();
    public:
        scan_state(std::wstring input);
        scan_state(const scan_state& state);
        virtual ~scan_state();

        // utilities
        wchar_t get_char();
        wchar_t get_char(int offset);
        void skip_whitespace();

        // scan methods

        // keywords
        // functions
        token try_scan_function_keyword();
        // loops
        // token try_scan_infinite_keyword();
        // token try_scan_break_keyword();
        // language features
        // token try_scan_feature_keyword();
        // token try_scan_autobreak_keyword();

        // brackets
        token try_scan_begin_scope_operator();
        token try_scan_end_scope_operator();

        // literals
        token try_scan_integer_literal();
        token try_scan_string_literal();
        token try_scan_identifier();
        token try_scan_float_literal();

        // boolean equality operators 
        token try_scan_boolean_eq_operator();
        token try_scan_boolean_ne_operator();

        // boolean and/or operators
        token try_scan_boolean_and_operator();
        token try_scan_boolean_or_operator();
        
        // boolean comparison operators
        token try_scan_boolean_gt_operator();
        token try_scan_boolean_lt_operator();
        token try_scan_boolean_gte_operator();
        token try_scan_boolean_lte_operator();

        // operators
        token try_scan_plus_operator();
        token try_scan_minus_operator();
        token try_scan_multiply_operator();
        token try_scan_divide_operator();
        token try_scan_concat_operator();
        token try_scan_assignment_operator();

        // parenthesis
        token try_scan_left_parenthesis();
        token try_scan_right_parenthesis();

        // program delimiters
        token try_scan_semicolon();
        token try_scan_list_delimiter();

        // file delimiters
        token try_scan_end_of_file();
        
        // scan delimiters
        bool out_of_range();
    };
    PTR_ALIAS(scan_state)
}

