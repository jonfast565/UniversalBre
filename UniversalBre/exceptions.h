#pragma once

#include <exception>
#include <string>

namespace exceptions {
    class argument_out_of_range_exception : 
        public std::exception {
    private:
        long _index = 0;
    public:
        void set_index(long index) { this->_index = index; }
        virtual const char* what() const throw() {
            auto except_str = "Index " + std::to_string(this->_index) + " out of bounds";
            return except_str.c_str();
        }
    };
}