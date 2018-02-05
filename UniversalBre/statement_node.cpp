#include "statement.h"

core::statement::statement(statement_type type): 
    type_(type)
{

}

core::statement_type core::statement::get_type() const
{
    return type_;
}
