#include "stdafx.h"

#include "log.h"
#include "scanner.h"
#include "console_log.h"
#include "program_header.h"
#include "console_waiter.h"
#include "console_handler.h"
#include "main.h"

int main()
{
    core::log *l = new core::console_log();
    l->set_console_font();
    print_header();
    auto *h = new core::console_handler();
    interactive_main(l, h);
    interactive_wait();
    return 0;
}

void print_header() {
    std::wcout << PROGRAM_HEADER;
}

void interactive_wait()
{
    auto *w = new core::console_waiter();
    w->wait();
}

void interactive_main(core::log* l, core::console_handler* h) {
    bool input_was_nil = false;
    while (!input_was_nil) {
        auto input = h->get_interactive_multiple_input();
        if (input.empty()) {
            input_was_nil = true;
            continue;
        }
        l->log_default(input);
        auto* s = new core::scanner(input);
    }
}

