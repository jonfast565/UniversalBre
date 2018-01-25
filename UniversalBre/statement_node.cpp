#include "statement_node.h"

core::statement_node::statement_node(statement_type type): 
    type_(type)
{

}

core::statement_type core::statement_node::get_type() const
{
    return type_;
}
