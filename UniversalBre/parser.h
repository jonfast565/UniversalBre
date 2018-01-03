#pragma once

#include "scanner.h"

namespace core {
    class parser
    {
    private:
        scanner & _s;
        scan_state _program_scan_state;
        void parse_program();
        void parse_expression();
        void parse_single_expression();
        void parse_addition_subtraction();
        void parse_multiplication_division();
    public:
        parser(scanner& s, std::wstring & input);
        virtual ~parser();
        void parse();
    };
}

