#pragma once

#include "global_defines.h"

namespace core
{
    template <typename T>
    class boxed_value
    {
        T value_;
    public:
        explicit boxed_value(T value);
        T get_value();
        void set_value(T value);
    };

    template<typename T>
    boxed_value<T>::boxed_value(T value) : value_(value)
    {
    }

    template <typename T>
    T boxed_value<T>::get_value()
    {
        return value_;
    }

    template <typename T>
    void boxed_value<T>::set_value(T value)
    {
        value_ = value;
    }
}
