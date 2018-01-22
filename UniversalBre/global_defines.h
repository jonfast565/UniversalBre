#pragma once

// Defines: Global newline constants (platform specific)
#ifdef _WIN32
#define NEWLINE "\r\n"
#define WIDE_NEWLINE L"\r\n"
#else
#define NEWLINE "\n"
#define WIDE_NEWLINE L"\n"
#endif

// Defines: max constant string length
#define MAX_STRING 100000

// Defines: default console font
#define DEFAULT_FONT L"Courier"

// Defines: program header
#define PROGRAM_HEADER core::header

// Defines: Windows colors
#ifdef _WIN32
#define WIN_BLUE 11
#define WIN_YELLOW 14
#define WIN_RED 12
#define WIN_WHITE 15
#define WIN_GREEN 2
#endif

// Includes: Windows/Linux C library subsystem includes
#ifdef _WIN32
#include <windows.h>
#include <conio.h>
#endif

// ... the rest ...
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <wchar.h>
#include <wctype.h>

// Includes: C++ library includes
#include <iostream>
#include <string>
#include <memory>
#include <iterator>
#include <vector>
#include <map>
#include <locale>
#include <codecvt>
#include <algorithm>
#include <cctype>
#include <cassert>
#include <locale>
#include <cstdarg>

// Functionality: Cause Windows to properly handle assert()
#ifdef EHA_SET

// define a function that throws
extern "C" void straight_to_debugger(unsigned int, EXCEPTION_POINTERS*)
{
    throw;
}

// set the intrinsic exception xlator to that one
extern "C" void(*old_translator)(unsigned, EXCEPTION_POINTERS*)
    = _set_se_translator(straight_to_debugger);

#endif

// ptr macros for coming up with using defs to shorten declarations
#define UNIQUE_PTR_ALIAS(x) using x##_ptr_u = std::unique_ptr<x>;
#define UNIQUE_VECPTR_ALIAS(x) using x##_vecptr_u = std::unique_ptr<std::vector<x>>;
#define UNIQUE_VECPTRPTR_ALIAS(x) using x##_vecptrptr_u = std::unique_ptr<std::vector<std::unique_ptr<x>>>;
#define SHARED_PTR_ALIAS(x) using x##_ptr_s = std::shared_ptr<x>;
#define SHARED_VECPTR_ALIAS(x) using x##_vecptr_s = std::shared_ptr<std::vector<x>>;
#define SHARED_VECPTRPTR_ALIAS(x) using x##_vecptrptr_s = std::shared_ptr<std::vector<std::shared_ptr<x>>>;

#define PTR_ALIAS(x) UNIQUE_PTR_ALIAS(x) \
                     UNIQUE_VECPTR_ALIAS(x) \
                     SHARED_PTR_ALIAS(x) \
                     SHARED_VECPTR_ALIAS(x) \
                     UNIQUE_VECPTRPTR_ALIAS(x) \
                     SHARED_VECPTRPTR_ALIAS(x)

# define VEC_ALIAS_1(x) using x##_vec = std::vector<x>;
# define VEC_ALIAS_2(x) using x##_ptr_vec_s = std::vector<std::shared_ptr<x>>;
# define VEC_ALIAS(x) VEC_ALIAS_1(x) \
                      VEC_ALIAS_2(x)

# define ALIAS_TYPES(x) PTR_ALIAS(x)\
                        VEC_ALIAS(x)

// Defines/Includes: add beneath this line
