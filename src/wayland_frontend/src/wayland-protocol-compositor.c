// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-compositor.h"
#include "wayland-protocol-surface.h"
#include "wayland-protocol-region.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

/// Handle destruction of compositor resource.
void noia_wayland_compositor_unbind(struct wl_resource* resource)
{
    LOG_WAYL2("Wayland: unbind compositor");
    noia_wayland_facade_remove_general_resource(NOIA_RESOURCE_OTHER, resource);
}

//------------------------------------------------------------------------------

/// Wayland protocol: create new surface.
void noia_wayland_create_surface(struct wl_client* client,
                                 struct wl_resource* resource,
                                 uint32_t id)
{
    NoiaSurfaceId new_sid = noia_wayland_facade_create_surface();
    int32_t version = wl_resource_get_version(resource);
    noia_wayland_surface_bind(client, (void*) new_sid, version, id);
}

//------------------------------------------------------------------------------

/// Wayland protocol: Create new region.
/// @see wayland-region.h
void noia_wayland_create_region(struct wl_client* client,
                                struct wl_resource* resource,
                                uint32_t id)
{
    NoiaItemId new_rid = noia_wayland_facade_create_region();
    int32_t version = wl_resource_get_version(resource);
    noia_wayland_region_bind(client, (void*) new_rid, version, id);
}

//------------------------------------------------------------------------------

const struct wl_compositor_interface scCompositorImplementation = {
        noia_wayland_create_surface,
        noia_wayland_create_region
    };

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request for interface to compositor object.
void noia_wayland_compositor_bind(struct wl_client* client,
                                  void* data,
                                  uint32_t version,
                                  uint32_t id)
{
    LOG_WAYL2("Binding Wayland compositor (version: %u, id: %u)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_compositor_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scCompositorImplementation,
                                   data, noia_wayland_compositor_unbind);

    noia_wayland_facade_add_general_resource(NOIA_RESOURCE_OTHER, rc);
}

//------------------------------------------------------------------------------

