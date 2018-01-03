#pragma once

#include "global_defines.h"
#include "scan_state.h"
#include "token.h"
#include "log.h"

namespace core {
    class scanner
    {
    private:
        log & _log_object;
    public:
        // ctor/dtor
        scanner(core::log& log_object);
        virtual ~scanner();
        
        // state fn
        scan_state get_initial_state(const std::wstring& input);

        // get one
        token scan_one(scan_state& state, token_type type);
    };
}

