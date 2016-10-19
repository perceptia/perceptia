// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_ENGINE_H
#define NOIA_WAYLAND_ENGINE_H

#include "global-enums.h"

#include "perceptia.h"

/// Allows control of Wayland display, thread and socket.
typedef struct NoiaWaylandEngineStruct NoiaWaylandEngine;

/// Allocate engine memory.
NoiaWaylandEngine* noia_wayland_engine_new(void);

/// Free engine memory.
void noia_wayland_engine_free(NoiaWaylandEngine* self);

/// Initialize engine.
NoiaResult noia_wayland_engine_initialize(NoiaWaylandEngine* self);

/// Finalize engine.
void noia_wayland_engine_finalize(NoiaWaylandEngine* self);

/// Register globals and start Wayland thread.
NoiaResult noia_wayland_engine_start(NoiaWaylandEngine* self);

/// Stops Wayland thread.
void noia_wayland_engine_stop(NoiaWaylandEngine* self);

/// Get next display serial.
int noia_wayland_engine_next_serial(NoiaWaylandEngine* self);

/// Add global Wayland object representing newly found output.
void noia_wayland_engine_advertise_output(NoiaWaylandEngine* self);

/// Remove global Wayland object representing output.
void noia_wayland_engine_destroy_output(NoiaWaylandEngine* self,
                                        NoiaOutput* output);

#endif // NOIA_WAYLAND_ENGINE_H

