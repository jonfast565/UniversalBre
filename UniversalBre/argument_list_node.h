#pragma once

#include "global_defines.h"
#include "token.h"

namespace core {
    class argument_list_node
    {
    private:
        token_vecptrptr_s arguments;
    public:
        argument_list_node() {}
        virtual ~argument_list_node() {}
        void add_argument(token_ptr_s argument);
    };
    ALIAS_TYPES(argument_list_node)
}

