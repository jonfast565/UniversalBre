#pragma once

#include "expression_node.h"
#include "op_types.h"

namespace core {
    class singleop_expression_node
        : public expression_node
    {
    private:
        expression_node_ptr_s _single_node = nullptr;
        op_type _temp_op_type = op_type::OP_INVALID;
    public:
        singleop_expression_node(
            expression_node_ptr_s single_node, 
            op_type temp_op_type = op_type::OP_INVALID)
            : _single_node(single_node), 
            _temp_op_type(temp_op_type), 
            expression_node(NODE_TYPE_SINGLE) {}
        ~singleop_expression_node() {}

        void print(int indent);

        op_type get_op_type();
        void set_op_type(op_type type);

        expression_node_ptr_s get_single_node();
        void set_single_node(expression_node_ptr_s node);
    };
    ALIAS_TYPES(singleop_expression_node)
}

