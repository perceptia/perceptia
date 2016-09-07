// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-surface.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

/// Handle destruction of surface resource.
void noia_wayland_surface_unbind(struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL2("Wayland: unbind surface (sid: %u)", sid);
    noia_wayland_facade_remove_surface(sid, resource);
}

//------------------------------------------------------------------------------

/// Handle destruction of frame resource.
void noia_wayland_surface_frame_unbind(struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL3("Wayland > unbind surface frame (sid: %u)", sid);
    noia_wayland_facade_remove_surface_resource
                                           (sid, NOIA_RESOURCE_FRAME, resource);
}

//------------------------------------------------------------------------------

/// Wayland protocol: destroy surface.
void noia_wayland_surface_destroy(struct wl_client* client     NOIA_UNUSED,
                                  struct wl_resource* resource NOIA_UNUSED)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL2("Wayland > destroy surface (sid: %u)", sid);
    wl_resource_destroy(resource);
}

//------------------------------------------------------------------------------

/// Wayland protocol: attach surface.
void noia_wayland_surface_attach(struct wl_client* client NOIA_UNUSED,
                                 struct wl_resource* resource,
                                 struct wl_resource* buffer_resource,
                                 int32_t sx, int32_t sy)
{
    int width = 0;
    int height = 0;
    int stride = 0;
    uint8_t* data = NULL;
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);

    LOG_WAYL3("Wayland > surface attach (sx: %d, sy: %d, sid: %d)",
              sx, sy, sid);

    struct wl_shm_buffer* shm_buffer = wl_shm_buffer_get(buffer_resource);
    if (shm_buffer) {
        width  = wl_shm_buffer_get_width(shm_buffer);
        height = wl_shm_buffer_get_height(shm_buffer);
        stride = wl_shm_buffer_get_stride(shm_buffer);
        data   = wl_shm_buffer_get_data(shm_buffer);
    } else {
        LOG_WARN3("Wayland: wrong shared memory buffer!");
    }

    noia_wayland_facade_surface_attach(sid, resource, buffer_resource,
                                       width, height, stride, data);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: damage surface.
void noia_wayland_surface_damage(struct wl_client* client NOIA_UNUSED,
                                 struct wl_resource* resource,
                                 int32_t x, int32_t y,
                                 int32_t width, int32_t height)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    LOG_WAYL4("Wayland > surface damage (x: %d, y: %d, w: %d, h: %d, sid: %u)",
              x, y, width, height, sid);
}

//------------------------------------------------------------------------------

/// Wayland protocol: subscribe for frame.
/// Client subscribes for one-shot notification about redraw of its surface.
void noia_wayland_surface_frame(struct wl_client* client,
                                struct wl_resource* resource,
                                uint32_t callback)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);

    LOG_WAYL3("Wayland > surface frame (cb: %d, sid: %d)", callback, sid);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_callback_interface, 1, callback);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, NULL, (void*) sid,
                                   noia_wayland_surface_frame_unbind);

    noia_wayland_facade_add_surface_resource(sid, NOIA_RESOURCE_FRAME, rc);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: set surface opaque region.
void noia_wayland_surface_set_opaque_region
                                          (struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource,
                                           struct wl_resource* region_resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    NoiaItemId rid = scInvalidItemId;
    if (region_resource) {
        rid = (NoiaItemId) wl_resource_get_user_data(region_resource);
    } else {
        /// @todo Clean opaque region.
    }

    LOG_NYIMP("Wayland > set opaque region (sid: %d, rid: %d)", sid, rid);
}

//------------------------------------------------------------------------------

/// Wayland protocol: set surface input region.
void noia_wayland_surface_set_input_region(struct wl_client* client NOIA_UNUSED,
                                           struct wl_resource* resource,
                                           struct wl_resource* region_resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);
    NoiaItemId rid = scInvalidItemId;
    if (region_resource) {
        rid = (NoiaItemId) wl_resource_get_user_data(region_resource);
    } else {
        /// @todo Clean input region.
    }

    LOG_WAYL3("Wayland > set input region (sid: %d, rid: %d)", sid, rid);

    noia_wayland_facade_set_input_region(sid, rid);
}

//------------------------------------------------------------------------------

/// Client tells compositor that all request were sent and the surface is now
/// ready to draw.
void noia_wayland_surface_commit(struct wl_client* client NOIA_UNUSED,
                                 struct wl_resource* resource)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) wl_resource_get_user_data(resource);

    LOG_WAYL3("Wayland > commit (sid: %d)", sid);

    noia_wayland_facade_commit(sid);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: set surface buffer transform.
void noia_wayland_surface_set_buffer_transform
                                      (struct wl_client* client     NOIA_UNUSED,
                                       struct wl_resource* resource NOIA_UNUSED,
                                       int32_t transform)
{
    LOG_NYIMP("Wayland > set buffer transform (transform: %d)", transform);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: set surface buffer scale.
void noia_wayland_surface_set_buffer_scale
                                      (struct wl_client* client     NOIA_UNUSED,
                                       struct wl_resource* resource NOIA_UNUSED,
                                       int32_t scale)
{
    LOG_NYIMP("Wayland > set buffer scale (scale: %d)", scale);
}

//------------------------------------------------------------------------------

/// @todo Wayland protocol: surface buffer damage.
void noia_wayland_surface_damage_buffer
                                      (struct wl_client* client     NOIA_UNUSED,
                                       struct wl_resource* resource NOIA_UNUSED,
                                       int32_t x,
                                       int32_t y,
                                       int32_t width,
                                       int32_t height)
{
    LOG_NYIMP("Wayland > damage surface buffer "
              "(x: '%d', y: '%d', width: '%d', height: '%d')",
              x, y, width, height);
}

//------------------------------------------------------------------------------

const struct wl_surface_interface scSurfaceImplementation = {
        noia_wayland_surface_destroy,
        noia_wayland_surface_attach,
        noia_wayland_surface_damage,
        noia_wayland_surface_frame,
        noia_wayland_surface_set_opaque_region,
        noia_wayland_surface_set_input_region,
        noia_wayland_surface_commit,
        noia_wayland_surface_set_buffer_transform,
        noia_wayland_surface_set_buffer_scale,
        noia_wayland_surface_damage_buffer,
    };

//------------------------------------------------------------------------------

void noia_wayland_surface_bind(struct wl_client* client,
                               void* data,
                               uint32_t version,
                               uint32_t id)
{
    NoiaSurfaceId sid = (NoiaSurfaceId) data;
    LOG_WAYL2("Binding Wayland surface "
              "(version: %u, id: %u, sid: %u)", version, id, sid);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_surface_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scSurfaceImplementation,
                                   data, noia_wayland_surface_unbind);

    noia_wayland_facade_add_surface(sid, rc);
}

//------------------------------------------------------------------------------

