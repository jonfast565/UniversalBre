#pragma once

#include "expression_node.h"
#include "binop_types.h"

namespace core {
    class binop_expression_node
        : public expression_node
    {
    private:
        expression_node_ptr_s _left_node = nullptr;
        expression_node_ptr_s _right_node = nullptr;
        binop_type _op_type;
    public:
        binop_expression_node(
            expression_node_ptr_s left_node,
            expression_node_ptr_s right_node,
            binop_type op_type) : 
            _left_node(left_node), _right_node(right_node), 
            _op_type(op_type) { }
        virtual ~binop_expression_node() {}
        void print(int indent);
        static expression_node_ptr_s fold_nodes(
            expression_node_ptr_s left_node,
            expression_node_ptr_s right_node);
        static expression_node_ptr_s fold_nodes(
            expression_node_ptr_s left_node,
            expression_node_ptr_s right_node,
            binop_type op_type);
        binop_type get_op_type();
    };
    PTR_ALIAS(binop_expression_node)
}

