// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-state.h"

#include "global-constants.h"

#include <malloc.h>

//------------------------------------------------------------------------------

NoiaWaylandState* noia_wayland_state_new()
{
    NoiaWaylandState* self = malloc(sizeof(*self));

    self->keyboard_state = noia_keyboard_state_new();
    self->keyboard_focused_sid = scInvalidItemId;
    self->pointer_focused_sid = scInvalidItemId;
    self->current_transfer = NULL;

    return self;
}

//------------------------------------------------------------------------------

void noia_wayland_state_initialize(NoiaWaylandState* self)
{
    noia_keyboard_state_initialize(self->keyboard_state);
}

//------------------------------------------------------------------------------

void noia_wayland_state_finalize(NoiaWaylandState* self)
{
    self->pointer_focused_sid = scInvalidItemId;
    self->keyboard_focused_sid = scInvalidItemId;

    noia_keyboard_state_finalize(self->keyboard_state);
}

//------------------------------------------------------------------------------

void noia_wayland_state_free(NoiaWaylandState* self)
{
    noia_keyboard_state_free(self->keyboard_state);
    free(self);
}

//------------------------------------------------------------------------------

