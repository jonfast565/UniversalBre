#pragma once

#include "global_defines.h"
#include "utility.h"

namespace core
{
    enum node_type
    {
        NODE_TYPE_BINARY,
        NODE_TYPE_SINGLE,
        NODE_TYPE_LITERAL
    };

    class expression_node
    {
    protected:
        ~expression_node() = default;
    private:
        node_type node_type_;
    public:
        explicit expression_node(node_type type) :
            node_type_(type)
        {
        }

        virtual void print(int indent) = 0;

        node_type get_node_type() const
        {
            return node_type_;
        }
    };

    ALIAS_TYPES(expression_node)
}
