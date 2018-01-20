#pragma once

#include "expression_node.h"
#include "literal_expression_node.h"
#include "op_types.h"

namespace core {
    class binop_expression_node : public expression_node
    {
    private:
        expression_node_ptr_s _left_node = nullptr;
        expression_node_ptr_s _right_node = nullptr;
        op_type _op_type;
    public:
        binop_expression_node(
            expression_node_ptr_s left_node,
            expression_node_ptr_s right_node,
            op_type op_type) : 
                expression_node(NODE_TYPE_BINARY),
                _left_node(left_node), 
                _right_node(right_node), 
                _op_type(op_type) {}
        virtual ~binop_expression_node() {}

        void print(int indent);
        bool one_node_populated();
        bool two_nodes_populated();
        bool left_node_populated();
        bool right_node_populated();

        expression_node_ptr_s get_left_node();
        void set_left_node(expression_node_ptr_s node);
        expression_node_ptr_s get_right_node();
        void set_right_node(expression_node_ptr_s node);
        op_type get_op_type();
        void set_op_type(op_type type);

        expression_node_ptr_s get_populated_node();
    };
    ALIAS_TYPES(binop_expression_node)
}

