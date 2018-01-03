#pragma once

#include "global_defines.h"
#include "scan_state.h"
#include "token.h"

namespace core {
    class scanner
    {
    private:
        scan_state get_initial_state(const std::wstring& input);
        token scan_one(scan_state& state);
    public:
        scanner() {};
        virtual ~scanner() {};
        std::vector<core::token> scan_all(const std::wstring& input);
    };
}

