#pragma once

namespace {
    bool is_interactive_mode(char ** argv, int argc);
    bool is_debug_mode(char ** argv, int argc);
    void interactive_wait();
    void print_header();
    void interactive_main(core::log_ptr_s l);
}
