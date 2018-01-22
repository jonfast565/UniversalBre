#include "argument_list_node.h"

core::argument_list_node::argument_list_node()
{
}

void core::argument_list_node::add_argument(const token_ptr_s argument) const
{
    arguments_->push_back(argument);
}
