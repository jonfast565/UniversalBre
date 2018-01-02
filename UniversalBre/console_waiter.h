#pragma once

#include <iostream>
#ifdef _WIN32
#include <conio.h>
#endif

namespace core {
    class console_waiter {
    public:
        console_waiter() {}
        virtual ~console_waiter() {}
        void wait() {
            std::cout << "Press any key to continue..." << std::endl;
            _getch();
        }
    };
}