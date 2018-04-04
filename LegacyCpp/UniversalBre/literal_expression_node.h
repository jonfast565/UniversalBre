#pragma once

#include "expression_node.h"

namespace core
{
    class literal_expression_node :
        public expression_node
    {
    private:
        std::wstring _value;
    public:
        literal_expression_node(std::wstring& value);;

        virtual ~literal_expression_node()
        {
        }

        void print(int indent) override;
    };

    ALIAS_TYPES(literal_expression_node)
}
