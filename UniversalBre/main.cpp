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
    auto l = core::console_log();
    l.set_console_font();
    print_header();

    if (argc == 2 && is_interactive_mode(argv)) {
        interactive_main(l);
        interactive_wait();
    }
    else {
        l.just_log(L"Operation not supported. Use the -i flag for interactive mode.");
    }
    return 0;
}

namespace {

    bool is_interactive_mode(char ** argv) 
    {
        return strcmp(argv[1], "-i") == 0;
    }

    void print_header() 
    {
        std::wcout << PROGRAM_HEADER;
    }

    void interactive_wait()
    {
        auto *w = new core::console_waiter();
        w->wait();
    }

    void interactive_main(core::log& l) {
        bool empty_input = false;
        auto h = core::console_handler();
        auto s = core::scanner(l);

        while (!empty_input) {
            auto input = h.get_interactive_multiple_input();

            if (input.empty()) {
                empty_input = true;
                continue;
            }

            l.log_default(input);

            try {
                // parse
            }
            catch (std::exception& e) {
                l.log_error(utility::cstr_to_wstring(e.what()));
            }
        }
    }
}

