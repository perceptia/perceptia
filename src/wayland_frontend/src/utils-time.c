// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "utils-time.h"

#include <time.h>
#include <unistd.h>

//------------------------------------------------------------------------------

NoiaMilliseconds noia_time_get_monotonic_milliseconds(void)
{
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (1000 * ts.tv_sec) + (ts.tv_nsec / 1000000);
}

//------------------------------------------------------------------------------

NoiaMilliseconds noia_time_get_realtime_milliseconds(void)
{
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (1000 * ts.tv_sec) + (ts.tv_nsec / 1000000);
}

//------------------------------------------------------------------------------

NoiaDayTime noia_time_get_local_daytime(void)
{
    struct tm* tm;
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    tm = localtime(&ts.tv_sec);

    NoiaDayTime dt;
    dt.hours = tm->tm_hour;
    dt.minutes = tm->tm_min;
    dt.seconds = tm->tm_sec;
    dt.useconds = ts.tv_nsec / 1000;
    return dt;
}

//------------------------------------------------------------------------------

void noia_time_snprintf(char* str, unsigned size, const char* format)
{
    struct tm* tm;
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    tm = localtime(&ts.tv_sec);

    strftime(str, size, format, tm);
}

//------------------------------------------------------------------------------

void noia_time_sleep(NoiaMilliseconds miliseconds)
{
    usleep(1000 * miliseconds);
}

//------------------------------------------------------------------------------

