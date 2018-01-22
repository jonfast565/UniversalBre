#pragma once

#include "global_defines.h"
#include "token.h"

namespace core
{
    class argument_list_node
    {
        token_vecptrptr_s arguments_;
    public:
        argument_list_node();
        void add_argument(token_ptr_s argument) const;
    };
    ALIAS_TYPES(argument_list_node)
}
