#pragma once

#ifdef _WIN32
#define NEWLINE "\r\n"
#define WIDE_NEWLINE L"\r\n"
#else
#define NEWLINE "\n"
#define WIDE_NEWLINE L"\n"
#endif