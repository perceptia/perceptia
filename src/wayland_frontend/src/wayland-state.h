// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_STATE_H
#define NOIA_WAYLAND_STATE_H

#include "utils-keyboard-state.h"
#include "wayland-transfer.h"

/// Structure containing current Wayland state.
/// @see NoiaWaylandContext NoiaWaylandContext
typedef struct {
    NoiaKeyboardState* keyboard_state;
    NoiaSurfaceId keyboard_focused_sid;
    NoiaSurfaceId pointer_focused_sid;
    NoiaWaylandTransfer* current_transfer;
} NoiaWaylandState;

/// Construct State.
NoiaWaylandState* noia_wayland_state_new(void);

/// Initialize State.
void noia_wayland_state_initialize(NoiaWaylandState* self);

/// Finalize State.
void noia_wayland_state_finalize(NoiaWaylandState* self);

/// Free State.
void noia_wayland_state_free(NoiaWaylandState* self);

#endif // NOIA_WAYLAND_STATE_H

