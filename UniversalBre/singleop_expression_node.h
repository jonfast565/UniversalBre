#pragma once

#include "expression_node.h"

namespace core {
    class singleop_expression_node
        : public expression_node
    {
    private:
        expression_node_ptr_s _single_node = nullptr;
    public:
        singleop_expression_node(expression_node_ptr_s single_node) 
            : _single_node(single_node) {}
        ~singleop_expression_node() {}
        void print(int indent);
    };
    PTR_ALIAS(singleop_expression_node)
}

