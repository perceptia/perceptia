// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-region.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

/// Handle destruction of region resource.
void noia_wayland_region_unbind(struct wl_resource* resource)
{
    NoiaItemId rid = (NoiaItemId) wl_resource_get_user_data(resource);
    LOG_WAYL3("Wayland: unbind region (rid: %d)", rid);
    noia_wayland_facade_remove_region(rid);
}

//------------------------------------------------------------------------------

/// Wayland protocol: destroy region.
void noia_wayland_region_destroy(struct wl_client* client     NOIA_UNUSED,
                                 struct wl_resource* resource NOIA_UNUSED)
{
    NoiaItemId rid = (NoiaItemId) wl_resource_get_user_data(resource);
    LOG_WAYL3("Wayland > region destroy (rid: %d)", rid);
    wl_resource_destroy(resource);
}

//------------------------------------------------------------------------------

/// Wayland protocol: add a square to a region.
/// Here concept is simplified.
/// @see wayland-region.h
void noia_wayland_region_add(struct wl_client* client NOIA_UNUSED,
                             struct wl_resource* resource,
                             int32_t x,
                             int32_t y,
                             int32_t width,
                             int32_t height)
{
    NoiaItemId rid = (NoiaItemId) wl_resource_get_user_data(resource);

    LOG_WAYL3("Wayland > region add (rid: %d, x: %d, y: %d, w: %d, h: %d)",
              rid, x, y, width, height);

    noia_wayland_facade_inflate_region(rid, x, y, width, height);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: subtract a square from a region.
/// @see wayland-region.h
void noia_wayland_region_subtract(struct wl_client* client     NOIA_UNUSED,
                                  struct wl_resource* resource NOIA_UNUSED,
                                  int32_t x,
                                  int32_t y,
                                  int32_t width,
                                  int32_t height)
{
    LOG_NYIMP("Wayland > region subtract (x: %d, y: %d, w: %d, h: %d)",
              x, y, width, height);
}

//------------------------------------------------------------------------------

const struct wl_region_interface scRegionImplementation = {
        noia_wayland_region_destroy,
        noia_wayland_region_add,
        noia_wayland_region_subtract
    };

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request for interface to region object.
void noia_wayland_region_bind(struct wl_client* client,
                              void* data,
                              uint32_t version,
                              uint32_t id)
{
    LOG_WAYL3("Binding Wayland region (version: %u, id: %u)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_region_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scRegionImplementation,
                                   data, noia_wayland_region_unbind);
}

//------------------------------------------------------------------------------

