#pragma once

#include "global_defines.h"
#include "utility.h"

namespace core
{
    enum class node_type
    {
        node_type_binary,
        node_type_single,
        node_type_literal
    };

    class expression_node
    {
        node_type node_type_;
    public:
        explicit expression_node(node_type type);
        virtual void print(int indent) = 0;
        node_type get_node_type() const;
    };

    inline expression_node::expression_node(node_type type):
        node_type_(type)
    {
    }

    inline node_type expression_node::get_node_type() const
    {
        return node_type_;
    }

    ALIAS_TYPES(expression_node)
}
