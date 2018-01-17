#pragma once

#include "expression_node.h"
#include "binop_types.h"

namespace core {
    class singleop_expression_node
        : public expression_node
    {
    private:
        expression_node_ptr_s _single_node = nullptr;
        binop_type _temp_op_type = OP_INVALID;
    public:
        singleop_expression_node(expression_node_ptr_s single_node, binop_type temp_op_type = OP_INVALID)
            : _single_node(single_node), 
            _temp_op_type(temp_op_type), 
            expression_node(NODE_TYPE_SINGLE) {}
        ~singleop_expression_node() {}

        void print(int indent);

        binop_type get_op_type();
        void set_op_type(binop_type type);

        expression_node_ptr_s get_single_node();
        void set_single_node(expression_node_ptr_s node);
    };
    PTR_ALIAS(singleop_expression_node)
}

