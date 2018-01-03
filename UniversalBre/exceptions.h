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
        extended_exception();
        virtual const char* what() const throw();
        virtual ~extended_exception();
    };

    // argument out of range exception 
    // for dealing with pesky array bounds
    class argument_out_of_range : 
        public extended_exception {
    public:
        argument_out_of_range(int index);
    };

    // scan failure: for when you crapped lexemes
    class scan_failure : 
        public extended_exception {
    public:
        explicit scan_failure(const wchar_t atom, const wchar_t* expected_atom);
        explicit scan_failure(const wchar_t* token_name, const wchar_t* expected_token_name);
        explicit scan_failure(const wchar_t* unrecognized_token);
    };
}