#pragma once

#include "global_defines.h"
#include "utility.h"

namespace exceptions {

    // extended exception for dealing with wide conversions
    class extended_exception :
        public std::exception {
    protected:
        char* except_str = nullptr;
        const wchar_t* except_wstr = nullptr;
    public:
        extended_exception() : std::exception() {
            except_str = new char[MAX_STRING];
        }
        virtual const char* what() const throw() {
            size_t i;
            wcstombs_s(&i, 
                this->except_str, 
                (size_t)MAX_STRING, 
                except_wstr, 
                wcslen(except_wstr));
            return except_str;
        }
        virtual ~extended_exception() {
            if (except_wstr != nullptr) {
                delete except_wstr;
            }
            if (except_str != nullptr) {
                delete except_str;
            }
        }
    };

    // argument out of range exception 
    // for dealing with pesky array bounds
    class argument_out_of_range : 
        public extended_exception {
    public:
        argument_out_of_range(int index) : 
            extended_exception() {
            utility::concat_in_place(
                &except_wstr, 3, 
                L"Indexed argument ", 
                std::to_wstring(index).c_str(), 
                L" is out of the range of the input.");
        }
    };

    // scan failure: for when you crapped lexemes
    class scan_failure : 
        public extended_exception {
    public:
        explicit scan_failure(const wchar_t atom, const wchar_t* expected_atom) :
            extended_exception() {
            utility::concat_in_place(
                &except_wstr, 5,
                L"Invalid atom '",
                (std::wstring() + atom).c_str(),
                L"', expected '",
                expected_atom,
                "'");
        }
        explicit scan_failure(const wchar_t* token_name, const wchar_t* expected_token_name) :
            extended_exception() {
            utility::concat_in_place(
                &except_wstr, 5,
                L"Invalid token '",
                token_name,
                L"', expected '",
                expected_token_name,
                "'");
        }
        explicit scan_failure(const wchar_t* unrecognized_token) :
            extended_exception() {
            utility::concat_in_place(
                &except_wstr, 5,
                L"Unrecognized token '",
                unrecognized_token,
                L"'");
        }
    };
}