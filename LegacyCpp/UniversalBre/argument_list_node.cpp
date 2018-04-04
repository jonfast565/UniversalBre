#include "argument_list_node.h"
#include "utility.h"

core::argument_list_node::argument_list_node()
{
    arguments_ = utility::make_ptr_s(std::vector<std::shared_ptr<token>>());
}

void core::argument_list_node::add_argument(const token_ptr_s argument) const
{
    arguments_->push_back(argument);
}
