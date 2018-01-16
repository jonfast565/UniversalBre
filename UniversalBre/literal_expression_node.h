#pragma once

#include "expression_node.h"

namespace core {
    class literal_expression_node :
        public expression_node
    {
    private:
        std::wstring _value = 0;
    public:
        literal_expression_node(std::wstring value) : _value(value) {};
        virtual ~literal_expression_node() {}
        void print(int indent);
    };
    PTR_ALIAS(literal_expression_node)
}

