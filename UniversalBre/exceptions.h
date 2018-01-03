#pragma once

#include <exception>
#include <string>

namespace exceptions {
    class argument_out_of_range : 
        public std::exception {
    private:
        long _index = 0;
    public:
        argument_out_of_range(int index) : _index(index) {}
        virtual const char* what() const throw() {
            auto except_str = "Index " + std::to_string(this->_index) + " out of bounds";
            return except_str.c_str(); // OMG didn't know this was total shit. Format in std::exception constructor with base()
        }
    };

    class scan_failure :
        public std::exception {
    private:
        std::string _token_name;
    public:
        scan_failure(std::wstring token_name) : _token_name(_token_name) {}
        virtual const char* what() const throw() {
            auto except_str = "Scan failure, tried " + this->_token_name;
            return except_str.c_str(); // OMG SRSLY
        }
    };
}