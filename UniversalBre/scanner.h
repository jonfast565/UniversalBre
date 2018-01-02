#pragma once

#include <iostream>
#include <memory>
#include "scan_state.h"

using ref_scan_state = std::unique_ptr<core::scan_state>;

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

