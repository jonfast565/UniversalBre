#pragma once
class code_generator
{
public:
    code_generator() {}
    ~code_generator() {}
    generate_code(expression_node_ptr_s starting_node);
};

