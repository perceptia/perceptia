// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_UTILS_TIME_H
#define NOIA_UTILS_TIME_H

#include "global-types.h"

/// Represents daytime.
typedef struct {
    int hours;    ///< Hours
    int minutes;  ///< Minutes
    int seconds;  ///< Seconds
    int useconds; ///< Microseconds
} NoiaDayTime;

/// Return number of milliseconds since arbitrary point in time.
NoiaMilliseconds noia_time_get_monotonic_milliseconds(void);

/// Return best guess for number of milliseconds since Epoch.
NoiaMilliseconds noia_time_get_realtime_milliseconds(void);

/// Return local time.
NoiaDayTime noia_time_get_local_daytime(void);

/// Prepare string containinge data and time.
/// @param str - string to be printed to
/// @param size - size of `str`
/// @param format - format string
/// @see strftime
void noia_time_snprintf(char* str, unsigned size, const char* format);

/// Sleep for given number of milliseconds.
void noia_time_sleep(NoiaMilliseconds miliseconds);

#endif // NOIA_UTILS_TIME_H

