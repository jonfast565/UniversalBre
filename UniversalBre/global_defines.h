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
#define MAX_STRING 10000

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
#endif

// Includes: Windows/Linux C library subsystem includes
#ifdef _WIN32
#include <windows.h>
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
#include <vector>
#include <locale>
#include <codecvt>
#include <algorithm>
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

// Defines/Includes: add beneath this line