#include "console_waiter.h"

void core::console_waiter::wait()
{
    std::cout << "Press any key to continue..." << std::endl;
    _getch();
}
