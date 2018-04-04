cmake_minimum_required(VERSION 3.9)

set(SOURCE_FILES
        # header files
        argument_list_node.h
        assignment_node.h
        atom_status.h
        binop_expression_node.h
        boxed_value.h
        code_generator.h
        console_handler.h
        console_log.h
        console_waiter.h
        exceptions.h
        expression_node.h
        function_expression_node.h
        global_defines.h
        literal_expression_node.h
        log.h
        main.h
        op_types.h
        parser.h
        printable.h
        program_header.h
        scan_state.h
        scanner.h
        semantic_types.h
        ssa_instruction.h
        statement.h
        statement_types.h
        token.h
        token_types.h
        utility.h
        # source files
        argument_list_node.cpp
        assignment_node.cpp
        atom_status.cpp
        binop_expression_node.cpp
        code_generator.cpp
        console_handler.cpp
        console_log.cpp
        console_waiter.cpp
        exceptions.cpp
        function_expression_node.cpp
        literal_expression_node.cpp
        main.cpp
        parser.cpp
        scan_state.cpp
        scanner.cpp
        ssa_instruction.cpp
        statement.cpp
        token.cpp
        utility.cpp)

add_executable(UniversalBre ${SOURCE_FILES})