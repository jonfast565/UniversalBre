#pragma once

#include "expression_node.h"
#include "literal_expression_node.h"
#include "op_types.h"

namespace core
{
    class binop_expression_node : public expression_node
    {
        expression_node_ptr_s left_node_ = nullptr;
        expression_node_ptr_s right_node_ = nullptr;
        op_type op_type_;
    public:
        binop_expression_node(
            expression_node_ptr_s left_node,
            expression_node_ptr_s right_node,
            op_type op_type);
        void print(int indent) override;
        bool one_node_populated() const;
        bool two_nodes_populated() const;
        bool left_node_populated() const;
        bool right_node_populated() const;
        expression_node_ptr_s get_left_node() const;
        void set_left_node(expression_node_ptr_s node);
        expression_node_ptr_s get_right_node() const;
        void set_right_node(expression_node_ptr_s node);
        op_type get_op_type() const;
        void set_op_type(op_type type);
        expression_node_ptr_s get_populated_node() const;
    };

    ALIAS_TYPES(binop_expression_node)
}
