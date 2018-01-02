#include "stdafx.h"

#include "log.h"
#include "console_log.h"
#include "console_waiter.h"
#include "console_handler.h"
#include "scanner.h"
#include "main.h"

int main()
{
    
    core::console_handler *h = new core::console_handler();
    std::wstring input = h->get_interactive_multiple_input();

    core::log *l = new core::console_log();
    l->log_default(input);

    core::scanner* s = new core::scanner(input);

    interactive_wait();
    return 0;
}

void interactive_wait()
{
    core::console_waiter *w = new core::console_waiter();
    w->wait();
}

