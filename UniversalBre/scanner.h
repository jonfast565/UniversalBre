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
        scan_state get_initial_state(const std::wstring& input);
    public:
        // ctor/dtor
        scanner(core::log& log_object) : _log_object(log_object) {};
        virtual ~scanner() {};
        
        // get one
        token scan_one(scan_state& state, token_type type);
    };
}

