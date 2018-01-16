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
        explicit scan_failure(const wchar_t unrecognized_atom);
        explicit scan_failure(const wchar_t* unrecognized_token);
    };

    // not implemented stuff
    class not_implemented_exception :
        public extended_exception {
    public:
        explicit not_implemented_exception(const wchar_t* not_implemented_thing);
    };

    // parse failure: for when you crapped tokens
    class parse_failure :
        public extended_exception {
    public:
        explicit parse_failure(const wchar_t* actual, const wchar_t* expected);
        explicit parse_failure(const wchar_t* rule_failure);
    };
}