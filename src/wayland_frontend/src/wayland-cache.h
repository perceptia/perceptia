// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#ifndef NOIA_WAYLAND_CACHE_H
#define NOIA_WAYLAND_CACHE_H

#include "wayland-region.h"
#include "wayland-surface.h"

#include "utils-list.h"

typedef struct NoiaWaylandCacheStruct NoiaWaylandCache;

/// Convince structure collecting Wayland recource and client.
typedef struct {
    struct wl_resource* rc;
    struct wl_client* cl;
} NoiaWaylandRc;

//------------------------------------------------------------------------------

/// Construct Cache.
NoiaWaylandCache* noia_wayland_cache_new(void);

/// Initialize Cache.
void noia_wayland_cache_initialize(NoiaWaylandCache* self);

/// Finalize Cache.
void noia_wayland_cache_finalize(NoiaWaylandCache* self);

/// Free Cache.
void noia_wayland_cache_free(NoiaWaylandCache* self);

//------------------------------------------------------------------------------

/// Lock access to cache.
void noia_wayland_cache_lock(NoiaWaylandCache* self);

/// Unlock access to cache.
void noia_wayland_cache_unlock(NoiaWaylandCache* self);

//------------------------------------------------------------------------------

/// Create and store new surface with given `sid` (surface ID).
void noia_wayland_cache_create_surface(NoiaWaylandCache* self,
                                       NoiaSurfaceId sid);

/// Return surface with given `sid` or `NULL` if not found.
NoiaWaylandSurface* noia_wayland_cache_find_surface(NoiaWaylandCache* self,
                                                    NoiaSurfaceId sid);

/// Remove surface with given `sid`.
void noia_wayland_cache_remove_surface(NoiaWaylandCache* self,
                                       NoiaSurfaceId sid);

//------------------------------------------------------------------------------

/// Create and store new region; return newly generated `rid` (region ID).
NoiaItemId noia_wayland_cache_create_region(NoiaWaylandCache* self);

/// Return region with given `rid` or `NULL` if not found.
NoiaWaylandRegion* noia_wayland_cache_find_region(NoiaWaylandCache* self,
                                                  NoiaItemId rid);

/// Remove region with given `rid`.
void noia_wayland_cache_remove_region(NoiaWaylandCache* self,
                                      NoiaItemId rid);

//------------------------------------------------------------------------------

/// Store surface resource.
void noia_wayland_cache_add_surface_resource
                                  (NoiaWaylandCache* self,
                                   NoiaSurfaceId sid,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource);

/// Store general resource.
void noia_wayland_cache_add_general_resource
                                  (NoiaWaylandCache* self,
                                   NoiaWaylandGeneralResourceType resource_type,
                                   struct wl_resource* resource);

/// Remove surface resource.
void noia_wayland_cache_remove_surface_resource
                                  (NoiaWaylandCache* self,
                                   NoiaSurfaceId sid,
                                   NoiaWaylandSurfaceResourceType resource_type,
                                   struct wl_resource* resource);

/// Remove general resource.
void noia_wayland_cache_remove_general_resource
                                  (NoiaWaylandCache* self,
                                   NoiaWaylandGeneralResourceType resource_type,
                                   struct wl_resource* resource);

/// Return given general resource.
NoiaList* noia_wayland_cache_get_resources
                                 (NoiaWaylandCache* self,
                                  NoiaWaylandGeneralResourceType resource_type);

/// Return surface resurce and client for given surface.
NoiaWaylandRc noia_wayland_cache_get_rc_for_sid(NoiaWaylandCache* self,
                                                NoiaSurfaceId sid);

//------------------------------------------------------------------------------

#endif // NOIA_WAYLAND_CACHE_H

