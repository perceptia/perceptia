// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_GLOBAL_MACROS_H
#define NOIA_GLOBAL_MACROS_H

#include "utils-debug.h"

#include <stdlib.h>
#include <stdio.h>

/// C++-style `and`.
#define and &&

/// C++-style `or`.
#define or ||

/// C++-style `not`.
#define not !

/// Mark variable as unused.
#define NOIA_UNUSED __attribute__((unused))

/// This statement helps to reduce number of `return`s making code more
/// predictable.
#define NOIA_BLOCK switch(0) default:

/// Get size of locally defined array.
#define NOIA_SIZEOF_ARRAY(a) (sizeof(a)/sizeof(*a))

/// In some places random numbers are used.
/// While debugging it is useful to have smaller numbers so result of random
/// number generator can be `&`-ed with this mask.
#ifdef DEBUG
    #define NOIA_RANDOM_MASK (0xFF)
#else
    #define NOIA_RANDOM_MASK (~0x0)
#endif

/// If condition `COND` is not fulfilled print an error and execute expression
/// `EXPR`.
///
/// Switched of when `NDEBUG` macro is defined so if `EXPR` is `abort()`,
/// it is equivalent to `assert()`.
#ifndef NOIA_ENSURE
    #ifndef NDEBUG
        #define NOIA_ENSURE(COND,EXPR) \
            if (not (COND)) { \
                noia_print_ensurence_failed(__LINE__, __FILE__, #COND); \
                EXPR; }
    #else
        #define NOIA_ENSURE(COND,EXPR) ((void) 0)
    #endif
#endif

#endif // NOIA_GLOBAL_MACROS_H

