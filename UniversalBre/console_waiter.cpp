#include "console_waiter.h"

void core::console_waiter::wait()
{
    std::cout << "Press any key to continue..." << std::endl;
#ifdef _WIN32
    _getch();
#else
    // TODO: Refrain from using system calls. Not only bad practice
    // makes the program really damn vulnerable
    system("read -n1 -p ' ' key");
#endif
}
