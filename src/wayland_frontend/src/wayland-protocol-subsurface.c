// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-subsurface.h"

#include "wayland-facade.h"

#include "global-macros.h"
#include "utils-log.h"

//------------------------------------------------------------------------------

/// @todo Handle destruction of subsurface resource.
void noia_wayland_subsurface_unbind(struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland: unbind subsurface");
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: destroy subsurface.
void noia_wayland_subsurface_destroy(struct wl_client* client     NOIA_UNUSED,
                                     struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland > subsurface destroy");
}

//------------------------------------------------------------------------------

/// Wayland protocol: set subsurface position.
void noia_wayland_subsurface_set_position(struct wl_client* client NOIA_UNUSED,
                                          struct wl_resource* resource,
                                          int32_t x, int32_t y)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL3("Wayland > subsurface set position "
              "(sid: %u, x: %d, y: %d)", sid, x, y);
    noia_wayland_facade_set_subsurface_position(sid, x, y);
}

//------------------------------------------------------------------------------

/// Wayland protocol: place subsurface above.
void noia_wayland_subsurface_place_above(struct wl_client* client NOIA_UNUSED,
                                         struct wl_resource* resource,
                                         struct wl_resource* sibling_resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    NoiaSurfaceId sibling_sid =
                    (NoiaSurfaceId) wl_resource_get_user_data(sibling_resource);
    LOG_WAYL3("Wayland > subsurface place above "
              "(sid: %u, sibling sid: %u)",
              sid, sibling_sid);

    noia_wayland_facade_reorder_satellites(sid, sibling_sid, true);
}

//------------------------------------------------------------------------------

/// Wayland protocol: place subsurface below.
void noia_wayland_subsurface_place_below(struct wl_client* client NOIA_UNUSED,
                                         struct wl_resource* resource,
                                         struct wl_resource* sibling_resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    NoiaSurfaceId sibling_sid =
                    (NoiaSurfaceId) wl_resource_get_user_data(sibling_resource);
    LOG_WAYL3("Wayland > subsurface place below "
              "(sid: %u, sibling sid: %u)",
              sid, sibling_sid);

    noia_wayland_facade_reorder_satellites(sid, sibling_sid, false);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: subsurface set sync.
void noia_wayland_subsurface_set_sync(struct wl_client* client     NOIA_UNUSED,
                                      struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland > subsurface set sync");
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: subsurface set desync.
void noia_wayland_subsurface_set_desync(struct wl_client* client     NOIA_UNUSED,
                                        struct wl_resource* resource NOIA_UNUSED)
{
    LOG_NYIMP("Wayland > subsurface set desync");
}

//------------------------------------------------------------------------------

const struct wl_subsurface_interface scSubsurfaceImplementation = {
        noia_wayland_subsurface_destroy,
        noia_wayland_subsurface_set_position,
        noia_wayland_subsurface_place_above,
        noia_wayland_subsurface_place_below,
        noia_wayland_subsurface_set_sync,
        noia_wayland_subsurface_set_desync
    };

//------------------------------------------------------------------------------

/// Wayland protocol: Handle request for interface to subsurface object.
void noia_wayland_subsurface_bind(struct wl_client* client,
                                  void* data,
                                  uint32_t version,
                                  uint32_t id)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) data;
    LOG_WAYL2("Binding Wayland subsurface "
              "(version: %u, id: %u, sid: %u)",
              version, id, sid);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_subsurface_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scSubsurfaceImplementation,
                                   data, noia_wayland_subsurface_unbind);
}

//------------------------------------------------------------------------------

