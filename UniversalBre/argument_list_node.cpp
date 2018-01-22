#include "argument_list_node.h"

void core::argument_list_node::add_argument(token_ptr_s argument)
{
    arguments->push_back(argument);
}
