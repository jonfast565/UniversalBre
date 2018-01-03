#include "exceptions.h"

exceptions::extended_exception::extended_exception() 
    : std::exception()
{
    except_str = new char[MAX_STRING];
}

const char * exceptions::extended_exception::what() 
const throw()
{
    size_t i;
    wcstombs_s(&i,
        this->except_str,
        (size_t)MAX_STRING,
        except_wstr,
        wcslen(except_wstr));
    return except_str;
}

exceptions::extended_exception::~extended_exception()
{
    if (except_wstr != nullptr) {
        delete except_wstr;
    }
    if (except_str != nullptr) {
        delete except_str;
    }
}

exceptions::argument_out_of_range::argument_out_of_range(int index) 
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr, 3,
        L"Indexed argument ",
        std::to_wstring(index).c_str(),
        L" is out of the range of the input.");
}

exceptions::scan_failure::scan_failure(const wchar_t atom, const wchar_t * expected_atom) 
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr, 5,
        L"Invalid atom '",
        (std::wstring() + atom).c_str(),
        L"', expected '",
        expected_atom,
        "'");
}

exceptions::scan_failure::scan_failure(const wchar_t * token_name, const wchar_t * expected_token_name)
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr, 5,
        L"Invalid token '",
        token_name,
        L"', expected '",
        expected_token_name,
        "'");
}

exceptions::scan_failure::scan_failure(const wchar_t * unrecognized_token)
    : extended_exception()
{
    utility::concat_in_place(
        &except_wstr, 5,
        L"Unrecognized token '",
        unrecognized_token,
        L"'");
}
