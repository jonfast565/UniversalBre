#pragma once

#include "expression_node.h"

namespace core {
    class binop_expression_node
        : public expression_node
    {
    private:
        expression_node_ptr_s _left_node = nullptr;
        expression_node_ptr_s _right_node = nullptr;
    public:
        binop_expression_node(
            expression_node_ptr_s left_node,
            expression_node_ptr_s right_node) : 
            _left_node(left_node), _right_node(right_node) { }
        virtual ~binop_expression_node() {}
        void print(int indent);
    };
    PTR_ALIAS(binop_expression_node)
}

