#include "scan_state.h"
#include "atom_status.h"

void core::scan_state::increment_location(int increment)
{
    location_ += increment;
    column_ += increment;
    input_left_ = input_.substr(location_, input_.length());
}

core::atom_status_ptr_s core::scan_state::get_char_atom()
{
    return utility::make_ptr_s(atom_status(get_char()));
}

core::scan_state::scan_state(std::wstring input)
{
    input_ = input;
    input_left_ = std::wstring(input);
}

core::scan_state::scan_state(const scan_state& state)
{
    column_ = state.column_;
    line_ = state.line_;
    location_ = state.location_;
    input_ = state.input_;
    input_left_ = state.input_left_;
}

wchar_t core::scan_state::get_char()
{
    if (location_ >= input_.size() || location_ < 0)
    {
        throw exceptions::argument_out_of_range(location_);
    }
    return input_[location_];
}

wchar_t core::scan_state::get_char(int offset)
{
    if (location_ + offset >= input_.size()
        || location_ + offset < 0)
    {
        throw exceptions::argument_out_of_range(location_ + offset);
    }
    return input_[location_ + offset];
}

void core::scan_state::skip_whitespace()
{
    if (out_of_range()) return;

    int temp_ctr = 0;
    wchar_t c = get_char(temp_ctr);
    if (!utility::is_whitespace(c)) return;

    while (true)
    {
        c = get_char(temp_ctr);
        if (utility::is_whitespace(c))
        {
            temp_ctr++;
            if (utility::is_newline_char(c))
            {
                line_++;
                column_ = 0;
            }
        }
        else
            break;
    }
    increment_location(temp_ctr);
}

core::token core::scan_state::try_scan_function_keyword()
{
    auto first_char = atom_status(get_char());
    if (!(first_char.get_atom() == L'f'
        || first_char.get_atom() == L'F'))
    {
        throw exceptions::scan_failure(get_char(), L"f");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (!(second_char.get_atom() != L'i'
        || second_char.get_atom() != L'I'))
    {
        throw exceptions::scan_failure(get_char(), L"i");
    }
    increment_location(1);
    auto third_char = atom_status(get_char());
    if (!(third_char.get_atom() != L'n'
        || third_char.get_atom() != L'N'))
    {
        throw exceptions::scan_failure(get_char(), L"n");
    }
    increment_location(1);
    auto space_post_char = atom_status(get_char());
    if (space_post_char.is_break_char() != true
        && space_post_char.is_empty_or_whitespace() != true)
    {
        throw exceptions::scan_failure(get_char(), L"identifier");
    }
    return token(token_type::function_keyword);
}

core::token core::scan_state::try_scan_begin_scope_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'{')
    {
        throw exceptions::scan_failure(get_char(), L"{");
    }
    increment_location(1);
    return token(token_type::scope_begin_operator);
}

core::token core::scan_state::try_scan_end_scope_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'}')
    {
        throw exceptions::scan_failure(get_char(), L"}");
    }
    increment_location(1);
    return token(token_type::scope_end_operator);
}

core::token core::scan_state::try_scan_integer_literal()
{
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.is_digit())
    {
        throw exceptions::scan_failure(get_char(), L"digit");
    }

    auto next_char = get_char_atom();
    do
    {
        if (next_char->is_digit())
        {
            result += get_char();
            increment_location(1);
        }
        else
        {
            throw exceptions::scan_failure(get_char(), L"digit");
        }
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any_integer());

    return token(token_type::integer_literal, result);
}

core::token core::scan_state::try_scan_string_literal()
{
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.breaks_any_string())
    {
        throw exceptions::scan_failure(get_char(), L"quotation");
    }

    auto next_char = get_char_atom();
    do
    {
        result += get_char();
        increment_location(1);
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any_string());

    result += get_char();
    increment_location(1);
    return token(token_type::string_literal, result);
}

core::token core::scan_state::try_scan_float_literal()
{
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.is_digit())
    {
        throw exceptions::scan_failure(get_char(), L"digit");
    }

    auto next_char = get_char_atom();
    bool precision_part = false;
    do
    {
        if (next_char->is_digit())
        {
            result += get_char();
            increment_location(1);
        }
        else if (next_char->is_dot() && precision_part == false)
        {
            result += get_char();
            increment_location(1);
            precision_part = true;
        }
        else
        {
            throw exceptions::scan_failure(get_char(), L"digit");
        }
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any_integer());

    return token(token_type::float_literal, result);
}

core::token core::scan_state::try_scan_boolean_eq_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'=')
    {
        throw exceptions::scan_failure(get_char(), L"=");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (second_char.get_atom() != L'=')
    {
        throw exceptions::scan_failure(get_char(), L"=");
    }
    increment_location(1);
    return token(token_type::boolean_eq_operator);
}

core::token core::scan_state::try_scan_boolean_ne_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'!')
    {
        throw exceptions::scan_failure(get_char(), L"!");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (second_char.get_atom() != L'=')
    {
        throw exceptions::scan_failure(get_char(), L"=");
    }
    increment_location(1);
    return token(token_type::boolean_ne_operator);
}

core::token core::scan_state::try_scan_boolean_and_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'&')
    {
        throw exceptions::scan_failure(get_char(), L"&");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (second_char.get_atom() != L'&')
    {
        throw exceptions::scan_failure(get_char(), L"&");
    }
    increment_location(1);
    return token(token_type::boolean_and_operator);
}

core::token core::scan_state::try_scan_boolean_or_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'|')
    {
        throw exceptions::scan_failure(get_char(), L"|");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (second_char.get_atom() != L'|')
    {
        throw exceptions::scan_failure(get_char(), L"|");
    }
    increment_location(1);
    return token(token_type::boolean_or_operator);
}

core::token core::scan_state::try_scan_boolean_gt_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'>')
    {
        throw exceptions::scan_failure(get_char(), L">");
    }
    increment_location(1);
    return token(token_type::boolean_gt_operator);
}

core::token core::scan_state::try_scan_boolean_lt_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'<')
    {
        throw exceptions::scan_failure(get_char(), L"<");
    }
    increment_location(1);
    return token(token_type::boolean_lt_operator);
}

core::token core::scan_state::try_scan_boolean_gte_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'>')
    {
        throw exceptions::scan_failure(get_char(), L">");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (second_char.get_atom() != L'=')
    {
        throw exceptions::scan_failure(get_char(), L"=");
    }
    increment_location(1);
    return token(token_type::boolean_gte_operator);
}

core::token core::scan_state::try_scan_boolean_lte_operator()
{
    auto first_char = atom_status(get_char());
    if (first_char.get_atom() != L'<')
    {
        throw exceptions::scan_failure(get_char(), L"<");
    }
    increment_location(1);
    auto second_char = atom_status(get_char());
    if (second_char.get_atom() != L'=')
    {
        throw exceptions::scan_failure(get_char(), L"=");
    }
    increment_location(1);
    return token(token_type::boolean_lte_operator);
}

core::token core::scan_state::try_scan_identifier()
{
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.is_identifier_char())
    {
        throw exceptions::scan_failure(get_char(), L"letter, digit, underscore");
    }

    auto next_char = get_char_atom();
    do
    {
        if (next_char->is_identifier_char())
        {
            result += get_char();
            increment_location(1);
        }
        else
        {
            throw exceptions::scan_failure(get_char(), L"letter, digit, underscore");
        }
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any());
    return token(token_type::identifier, result);
}

core::token core::scan_state::try_scan_plus_operator()
{
    if (get_char() == L'+')
    {
        increment_location(1);
        return token(token_type::plus_operator);
    }
    throw exceptions::scan_failure(get_char(), L"plus operator");
}

core::token core::scan_state::try_scan_minus_operator()
{
    if (get_char() == L'-')
    {
        increment_location(1);
        return token(token_type::minus_operator);
    }
    throw exceptions::scan_failure(get_char(), L"minus operator");
}

core::token core::scan_state::try_scan_multiply_operator()
{
    if (get_char() == L'*')
    {
        increment_location(1);
        return token(token_type::multiply_operator);
    }
    throw exceptions::scan_failure(get_char(), L"multiply operator");
}

core::token core::scan_state::try_scan_divide_operator()
{
    if (get_char() == L'/')
    {
        increment_location(1);
        return token(token_type::divide_operator);
    }
    throw exceptions::scan_failure(get_char(), L"divide operator");
}

core::token core::scan_state::try_scan_concat_operator()
{
    if (get_char() == L'~')
    {
        increment_location(1);
        return token(token_type::concat_operator);
    }
    throw exceptions::scan_failure(get_char(), L"concat operator");
}

core::token core::scan_state::try_scan_assignment_operator()
{
    if (get_char() == L'=')
    {
        increment_location(1);
        return token(token_type::assignment_operator);
    }
    throw exceptions::scan_failure(get_char(), L"assignment operator");
}

core::token core::scan_state::try_scan_left_parenthesis()
{
    if (get_char() == L'(')
    {
        increment_location(1);
        return token(token_type::left_parenthesis);
    }
    throw exceptions::scan_failure(get_char(), L"left parenthesis");
}

core::token core::scan_state::try_scan_right_parenthesis()
{
    if (get_char() == L')')
    {
        increment_location(1);
        return token(token_type::right_parenthesis);
    }
    throw exceptions::scan_failure(get_char(), L"right parenthesis");
}

core::token core::scan_state::try_scan_semicolon()
{
    if (get_char() == L';')
    {
        increment_location(1);
        return token(token_type::semicolon);
    }
    throw exceptions::scan_failure(get_char(), L"semicolon");
}

core::token core::scan_state::try_scan_list_delimiter()
{
    if (get_char() == L',')
    {
        increment_location(1);
        return token(token_type::list_delimiter);
    }
    throw exceptions::scan_failure(get_char(), L"list delimiter");
}

core::token core::scan_state::try_scan_end_of_file()
{
    if (out_of_range())
    {
        return token(token_type::end_of_file);
    }
    throw exceptions::scan_failure(get_char(), L"end of file");
}

bool core::scan_state::out_of_range()
{
    return location_ >= input_.size() - 1 || location_ < 0;
}

core::scan_state::~scan_state()
{
}
