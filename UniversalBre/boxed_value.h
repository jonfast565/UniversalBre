#pragma once

#include "global_defines.h"

namespace core {
    template <typename T>
    class boxed_value
    {
    private:
        T _value;
    public:
        boxed_value(T value) : _value(value) {}
        ~boxed_value() {}
        T get_value();
        void set_value(T value);
    };

    template<typename T>
    inline T boxed_value<T>::get_value()
    {
        return _value;
    }

    template<typename T>
    inline void boxed_value<T>::set_value(T value)
    {
        _value = value;
    }
    ALIAS_TYPES(boxed_value)
}
