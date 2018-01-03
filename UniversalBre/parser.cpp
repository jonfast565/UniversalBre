#include "parser.h"

core::parser::parser(scanner & s, std::wstring & input) :  _s(s),
    _program_scan_state(_s.get_initial_state(input))
{
}

core::parser::~parser()
{
}

void core::parser::parse()
{
    parse_program();
}

void core::parser::parse_program()
{
    parse_expression();
}

void core::parser::parse_expression()
{
    switch (_program_scan_state.get_char()) {
    case L'(':
        _program_scan_state.scan_left_parenthesis();
        parse_expression();
        _program_scan_state.scan_right_parenthesis();
        break;
    default:
        parse_single_expression();
    }
    _program_scan_state.scan_end_of_file();
}

void core::parser::parse_single_expression()
{
    _program_scan_state.scan_integer_literal();
    switch (_program_scan_state.get_char()) {
    case L'+':
        _program_scan_state.scan_plus_operator();
        break;
    case L'-':
        _program_scan_state.scan_minus_operator();
        break;
    default:
        // TODO: should be parse exception type
        throw std::exception("Missing operand");
    }
    _program_scan_state.scan_integer_literal();
}

void core::parser::parse_addition_subtraction()
{

}

void core::parser::parse_multiplication_division()
{

}