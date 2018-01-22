#pragma once

#include "global_defines.h"
#include "scan_state.h"
#include "token.h"
#include "log.h"

namespace core
{
    class scanner
    {
    private:
        // scan logging
        log_ptr_s _log_object;

        // initial scanning + state ops
        token scan_one(scan_state_ptr_s state);
        scan_state get_initial_state(const std::wstring& input);
    public:
        // ctor/dtor
        scanner(log_ptr_s log_object);
        virtual ~scanner();

        // scan and provide a vector
        token_vecptr_s scan_all(const std::wstring& input);
    };

    ALIAS_TYPES(scanner)
}
