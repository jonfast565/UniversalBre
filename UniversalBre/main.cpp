#include "global_defines.h"
#include "log.h"
#include "utility.h"
#include "scanner.h"
#include "console_log.h"
#include "program_header.h"
#include "console_waiter.h"
#include "console_handler.h"
#include "token.h"
#include "main.h"

int main(int argc, char* argv[])
{
    core::log *l = new core::console_log();
    l->set_console_font();
    print_header();

    if (argc == 2 && strcmp(argv[1], "-i") == 0) {
        interactive_main(l);
        interactive_wait();
    }
    else {
        l->just_log(L"Operation not supported. Use the -i flag for interactive mode.");
    }
    return 0;
}

namespace {
    void print_header() {
        std::wcout << PROGRAM_HEADER;
    }

    void interactive_wait()
    {
        auto *w = new core::console_waiter();
        w->wait();
    }

    void interactive_main(core::log* l) {
        bool empty_input = false;
        auto *h = new core::console_handler();
        auto* s = new core::scanner();

        while (!empty_input) {
            auto input = h->get_interactive_multiple_input();

            if (input.empty()) {
                empty_input = true;
                continue;
            }

            l->log_default(input);

            try {
                auto token_list = s->scan_all(input);
                for (core::token& t : token_list) {
                    l->just_log(t.get_type() + L": " + t.get_lexeme());
                }
            }
            catch (std::exception& e) {
                l->log_error(utility::cstr_to_wstring(e.what()));
            }
        }
    }
}

