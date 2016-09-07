// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-context.h"

//------------------------------------------------------------------------------

NoiaWaylandContext* noia_wayland_context_new(void)
{
    NoiaWaylandContext* self = malloc(sizeof(*self));
    self->engine = noia_wayland_engine_new();
    self->cache = noia_wayland_cache_new();
    self->state = noia_wayland_state_new();
    return self;
}

//------------------------------------------------------------------------------

NoiaResult noia_wayland_context_initialize(NoiaWaylandContext* self,
                                           NoiaCoordinator* coordinator)
{
    NOIA_ENSURE(self, return NOIA_RESULT_INCORRECT_ARGUMENT);

    NoiaResult r = noia_wayland_engine_initialize(self->engine);
    if (r == NOIA_RESULT_SUCCESS) {
        self->coordinator = coordinator;
        noia_wayland_cache_initialize(self->cache);
        noia_wayland_state_initialize(self->state);

        r = noia_wayland_engine_start(self->engine);
    }

    return r;
}

//------------------------------------------------------------------------------

void noia_wayland_context_finalize(NoiaWaylandContext* self)
{
    NOIA_ENSURE(self, return);

    noia_wayland_engine_stop(self->engine);

    noia_wayland_state_finalize(self->state);
    noia_wayland_cache_finalize(self->cache);
    noia_wayland_engine_finalize(self->engine);
}

//------------------------------------------------------------------------------

void noia_wayland_context_free(NoiaWaylandContext* self)
{
    noia_wayland_state_free(self->state);
    noia_wayland_cache_free(self->cache);
    noia_wayland_engine_free(self->engine);
    free(self);
}

//------------------------------------------------------------------------------

