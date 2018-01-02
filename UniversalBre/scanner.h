#pragma once

#include <iostream>
#include "scan_state.h"

namespace core {
    class scanner
    {
    private:
        ref_scan_state get_initial_state(const std::wstring& input);
    public:
        scanner() {};
        virtual ~scanner() {};
    };
}

