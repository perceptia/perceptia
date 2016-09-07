// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_OUTPUT_H
#define NOIA_WAYLAND_OUTPUT_H

#include "utils-store.h"
#include "perceptia.h"

#include <wayland-server.h>

typedef struct {
    NoiaItem base;
    struct wl_global* global_output;
    NoiaOutput* output;
} NoiaWaylandOutput;

NoiaWaylandOutput* noia_wayland_output_create(struct wl_global* global_output,
                                              NoiaOutput* output);

void noia_wayland_output_destroy(NoiaWaylandOutput* self);

#endif // NOIA_WAYLAND_OUTPUT_H

