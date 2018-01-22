#include "exceptions.h"

exceptions::extended_exception::extended_exception()
{
    except_str_ = new char[MAX_STRING];
}

const char* exceptions::extended_exception::what()
const noexcept
{
    size_t i;
    wcstombs_s(&i,
               this->except_str_,
               static_cast<size_t>(MAX_STRING),
               except_wstr_,
               wcslen(except_wstr_));
    return except_str_;
}

exceptions::extended_exception::~extended_exception()
{
    delete except_wstr_;
    delete except_str_;
}

exceptions::argument_out_of_range::argument_out_of_range(int index)
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr_, 3,
        L"Indexed argument ",
        std::to_wstring(index).c_str(),
        L" is out of the range of the input.");
}

exceptions::scan_failure::scan_failure(const wchar_t atom, const wchar_t* expected_atom)
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr_, 5,
        L"Invalid atom '",
        (std::wstring() + atom).c_str(),
        L"', expected '",
        expected_atom,
        "'");
}

exceptions::scan_failure::scan_failure(const wchar_t unrecognized_atom)
{
    utility::concat_in_place(
        &except_wstr_, 3,
        L"Unrecognized atom '",
        (std::wstring() + unrecognized_atom).c_str(),
        L"'");
}

exceptions::scan_failure::scan_failure(const wchar_t* unrecognized_token)
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr_, 3,
        L"Unrecognized token '",
        unrecognized_token,
        L"'");
}

exceptions::not_implemented_exception::not_implemented_exception(const wchar_t* not_implemented_thing)
{
    utility::concat_in_place(
        &except_wstr_, 3,
        L"'",
        not_implemented_thing,
        L"' is not implemented");
}

exceptions::parse_failure::parse_failure(const wchar_t* actual, const wchar_t* expected)
{
    utility::concat_in_place(
        &except_wstr_, 5,
        L"Got '",
        actual,
        L"' expected '",
        expected,
        L"'");
}

exceptions::parse_failure::parse_failure(const wchar_t* rule_failure)
{
    utility::concat_in_place(
        &except_wstr_, 1,
        rule_failure);
}
