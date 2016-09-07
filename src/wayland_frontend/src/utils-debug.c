// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "utils-debug.h"

#include <stdio.h>
#include <execinfo.h>

#define __USE_GNU
#include <dlfcn.h>

//------------------------------------------------------------------------------

NoiaDebugConfig* noia_debug_config(void)
{
    static NoiaDebugConfig config = {printf, noia_print_backtrace, NULL};
    return &config;
}

//------------------------------------------------------------------------------

void noia_print_ensurence_failed(int line,
                                 const char* filename,
                                 const char* condition)
{
    fprintf(stderr, "Noia: %s: %d: Ensurence '%s' failed!\n",
            filename, line, condition);

    if (noia_debug_config()->print_failure) {
        noia_debug_config()->print_failure(line, filename, condition);
    }

    if (noia_debug_config()->print_backtrace) {
        noia_debug_config()->print_backtrace();
    }
}

//------------------------------------------------------------------------------

int noia_print_backtrace(void)
{
    int n = 0;
    Dl_info info;
    void* array[128];
    size_t size = backtrace(array, sizeof(array));

    NoiaPrintFunc print = printf;
    if (noia_debug_config()->print) {
        print = noia_debug_config()->print;
    }

    for (size_t i = 0; i < size; i++) {
        dladdr(array[i], &info);
        n += print("%015lx | %-45s | %s\n",
                   (long) array[i],
                   info.dli_fname ? info.dli_fname : "???",
                   info.dli_sname ? info.dli_sname : "---");
    }

    return n;
}

//------------------------------------------------------------------------------

