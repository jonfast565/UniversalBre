#pragma once

#include "expression_node.h"

namespace core {
    class literal_expression_node :
        public expression_node
    {
    private:
        std::wstring _value;
    public:
        literal_expression_node(std::wstring& value) : 
            expression_node(NODE_TYPE_LITERAL), 
            _value(value) {};
        virtual ~literal_expression_node() {}
        void print(int indent);
    };
    PTR_ALIAS(literal_expression_node)
}

