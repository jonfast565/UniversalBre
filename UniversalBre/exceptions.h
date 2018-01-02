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
        const char* what() const throw() {
            auto except_str = "Index " + std::to_string(this->_index) + " out of bounds";
            return except_str.c_str();
        }
    };

    class scan_failure :
        public std::exception {
    private:
        std::string _token_name;
    public:
        scan_failure(std::string token_name) : _token_name(_token_name) {}
        const char* what() const throw() {
            auto except_str = "Scan failure, tried " + _token_name;
            return except_str.c_str();
        }
    };
}