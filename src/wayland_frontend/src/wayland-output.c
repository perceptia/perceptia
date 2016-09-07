// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-output.h"

#include "utils-log.h"

#include <malloc.h>
#include <memory.h>

//------------------------------------------------------------------------------

NoiaWaylandOutput* noia_wayland_output_create(struct wl_global* global_output,
                                              NoiaOutput* output)
{
    NoiaWaylandOutput* self = malloc(sizeof(NoiaWaylandOutput));
    if (!self) {
        LOG_ERROR("Memory allocation failure");
        return self;
    }

    self->base.str = NULL;
    self->global_output = global_output;
    self->output = output;
    return self;
}

//------------------------------------------------------------------------------

void noia_wayland_output_destroy(NoiaWaylandOutput* self)
{
    if (!self) {
        return;
    }

    noia_output_unref(self->output);
    memset(self, 0, sizeof(NoiaWaylandOutput));
    free(self);
}

//------------------------------------------------------------------------------

