#pragma once

#include <iostream>
#include <memory>
#include "scan_state.h"

namespace core {
    class scanner
    {
    private:
        std::unique_ptr<scan_state> initial_state;
        std::unique_ptr<scan_state> get_initial_state();
    public:
        scanner(const std::wstring& input);
        ~scanner();
    };
}

