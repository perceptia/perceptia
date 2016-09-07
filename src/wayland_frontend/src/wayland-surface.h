// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_SURFACE_H
#define NOIA_WAYLAND_SURFACE_H

#include "wayland-types.h"
#include "utils-list.h"

#include <wayland-server.h>

/// Structure used by cache for storing surface-related resources.
typedef struct NoiaWaylandSurfaceStruct NoiaWaylandSurface;

/// Wayland surface constructor.
NoiaWaylandSurface* noia_wayland_surface_new(void);

/// Wayland surface destructor.
void noia_wayland_surface_free(NoiaWaylandSurface* self);

/// Get resource of given type.
struct wl_resource* noia_wayland_surface_get_resource
                                 (NoiaWaylandSurface* self,
                                  NoiaWaylandSurfaceResourceType resource_type);

/// Get list of frame resources.
/// @note Qt applications use two frames.
NoiaList* noia_wayland_surface_get_frame_resources(NoiaWaylandSurface* self);

/// Add resource of given type.
void noia_wayland_surface_add_resource
                                  (NoiaWaylandSurface* self,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource);

/// Remove resource of given type.
void noia_wayland_surface_remove_resource
                                  (NoiaWaylandSurface* self,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource);

#endif // NOIA_WAYLAND_SURFACE_H

