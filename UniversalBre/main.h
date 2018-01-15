#pragma once

namespace {
    bool is_interactive_mode(char ** argv);
    void interactive_wait();
    void print_header();
    void interactive_main(core::log_ptr_s l);
}
