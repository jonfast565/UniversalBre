#include "global_defines.h"
#include "log.h"
#include "utility.h"
#include "scanner.h"
#include "console_log.h"
#include "program_header.h"
#include "console_waiter.h"
#include "console_handler.h"
#include "code_generator.h"
#include "token.h"
#include "parser.h"
#include "main.h"

int main(int argc, char* argv[])
{
    core::log_ptr_s l = utility::make_ptr_s(core::console_log(true));
    l->set_console_font();
    print_header();

    if (argc == 3 && is_interactive_mode(argv, 1) && is_debug_mode(argv, 2))
    {
        l->set_debug(true);
        interactive_main(l);
        interactive_wait();
    }
    else if (argc == 2 && is_interactive_mode(argv, 1))
    {
        l->set_debug(false);
        interactive_main(l);
        interactive_wait();
    }
    else
    {
        l->just_log(
            L"Operation not supported. Use the -i flag for interactive mode, then -d afterwards to enable debugging.");
    }

    return 0;
}

namespace
{
    bool is_interactive_mode(char** argv, int argc)
    {
        return strcmp(argv[argc], "-i") == 0;
    }

    bool is_debug_mode(char** argv, int argc)
    {
        return strcmp(argv[argc], "-d") == 0;
    }

    void print_header()
    {
        std::wcout << PROGRAM_HEADER;
    }

    void interactive_wait()
    {
        auto w = utility::make_ptr_u(core::console_waiter());
        w->wait();
    }

    void interactive_main(core::log_ptr_s l)
    {
        // handlers
        auto h = utility::make_ptr_u(core::console_handler());

        // check for input, empty exits
        bool empty_input = false;
        while (!empty_input)
        {
            auto input = h->get_interactive_multiple_input();
            if (input.empty())
            {
                empty_input = true;
                continue;
            }

            l->log_default(input);

            try
            {
                auto s = core::scanner(l);
                auto v = s.scan_all(input);
                auto p = core::parser(v, l);
                auto expr = p.parse();
                auto g = core::code_generator();
                auto instrs = g.generate_code();
                /*
                for (auto &i : *instrs) {
                    i.print_code();
                }*/
            }
            catch (std::exception& e)
            {
                l->log_error(utility::cstr_to_wstring(e.what()));
            }
        }
    }
}