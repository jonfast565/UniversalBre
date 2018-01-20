#pragma once

#include "global_defines.h"
#include "utility.h"

namespace core {
    enum node_type {
        NODE_TYPE_BINARY,
        NODE_TYPE_SINGLE,
        NODE_TYPE_LITERAL
    };
    class expression_node {
    private:
        node_type _node_type;
    public:
        expression_node(node_type type) : 
            _node_type(type) {}

        virtual void print(int indent) = 0;

        node_type get_node_type() {
            return _node_type;
        }
    };
    ALIAS_TYPES(expression_node)
}