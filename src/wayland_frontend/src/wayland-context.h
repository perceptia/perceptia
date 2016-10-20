// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_CONTEXT_H
#define NOIA_WAYLAND_CONTEXT_H

#include "wayland-engine.h"
#include "wayland-cache.h"
#include "wayland-state.h"

#include "perceptia.h"

/// Global context of Wayland.
/// @see NoiaWaylandEngine NoiaWaylandCache NoiaWaylandState
typedef struct {
    NoiaWaylandEngine* engine;
    NoiaWaylandCache* cache;
    NoiaWaylandState* state;
    NoiaCoordinator* coordinator;
    NoiaKeymapSettings keymap_settings;
} NoiaWaylandContext;

/// Construct Context.
NoiaWaylandContext* noia_wayland_context_new(NoiaKeymapSettings* keymap_settings);

/// Initialize Context.
NoiaResult noia_wayland_context_initialize(NoiaWaylandContext* self,
                                           NoiaCoordinator* coordinator);

/// Finalize Context.
void noia_wayland_context_finalize(NoiaWaylandContext* self);

/// Free Context.
void noia_wayland_context_free(NoiaWaylandContext* self);

#endif // NOIA_WAYLAND_CONTEXT_H

