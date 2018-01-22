#include "binop_expression_node.h"

void core::binop_expression_node::print(int indent)
{
    if (left_node_ == nullptr)
    {
        std::wcout
            << indent << " "
            << utility::build_indent_str(indent + 1)
            << "N/A"
            << std::endl;
    }
    else
    {
        left_node_->print(indent + 1);
    }

    std::wcout
        << indent << " "
        << utility::build_indent_str(indent)
        << get_op_type_string(op_type_)
        << std::endl;

    if (right_node_ == nullptr)
    {
        std::wcout
            << indent << " "
            << utility::build_indent_str(indent + 1)
            << "N/A"
            << std::endl;
    }
    else
    {
        right_node_->print(indent + 1);
    }
}

core::op_type core::binop_expression_node::get_op_type() const
{
    return op_type_;
}

void core::binop_expression_node::set_op_type(op_type type)
{
    op_type_ = type;
}

bool core::binop_expression_node::one_node_populated() const
{
    return (left_node_ == nullptr
        && right_node_ != nullptr)
        || (left_node_ != nullptr
        && right_node_ == nullptr);
}

bool core::binop_expression_node::two_nodes_populated() const
{
    return left_node_ != nullptr && right_node_ != nullptr;
}

bool core::binop_expression_node::left_node_populated() const
{
    return left_node_ != nullptr;
}

bool core::binop_expression_node::right_node_populated() const
{
    return right_node_ != nullptr;
}

core::expression_node_ptr_s core::binop_expression_node::get_left_node() const
{
    assert(left_node_ != nullptr);
    return left_node_;
}

core::expression_node_ptr_s core::binop_expression_node::get_right_node() const
{
    assert(right_node_ != nullptr);
    return right_node_;
}

core::expression_node_ptr_s core::binop_expression_node::get_populated_node() const
{
    assert(one_node_populated());
    if (left_node_populated())
    {
        return left_node_;
    }
    if (right_node_populated())
    {
        return right_node_;
    }

    // SHOULD NEVER HAPPEN (assert)
    return nullptr;
}

void core::binop_expression_node::set_left_node(const expression_node_ptr_s node)
{
    left_node_ = node;
}

void core::binop_expression_node::set_right_node(const expression_node_ptr_s node)
{
    right_node_ = node;
}
