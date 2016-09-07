// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#include "wayland-protocol-keyboard.h"
#include "wayland-facade.h"

#include "utils-log.h"
#include "global-macros.h"

//------------------------------------------------------------------------------

void noia_wayland_pointer_unbind(struct wl_resource* resource)
{
    LOG_WAYL3("Wayland: unbind pointer");
    noia_wayland_facade_remove_general_resource(NOIA_RESOURCE_POINTER, resource);
}

//------------------------------------------------------------------------------

void noia_wayland_pointer_set_cursor(struct wl_client* client     NOIA_UNUSED,
                                     struct wl_resource* resource NOIA_UNUSED,
                                     uint32_t serial,
                                     struct wl_resource* surface_resource,
                                     int32_t hotspot_x,
                                     int32_t hotspot_y)
{
    if (surface_resource == NULL) {
        return;
    }

    NoiaSurfaceId sid =
                    (NoiaSurfaceId) wl_resource_get_user_data(surface_resource);

    LOG_WAYL3("Wayland: set cursor "
              "(serial: %d, hotspot_x: %d, hotspot_y: %d, sid: %d)",
              serial, hotspot_x, hotspot_y, sid);

    noia_wayland_facade_set_cursor(serial, hotspot_x, hotspot_y, sid);
}

//------------------------------------------------------------------------------

void noia_wayland_pointer_release(struct wl_client* client NOIA_UNUSED,
                                  struct wl_resource* resource)
{
    LOG_WAYL2("Wayland: pointer release");
    wl_resource_destroy(resource);
}

//------------------------------------------------------------------------------

const struct wl_pointer_interface scPointerImplementation = {
        noia_wayland_pointer_set_cursor,
        noia_wayland_pointer_release
    };

//------------------------------------------------------------------------------

void noia_wayland_pointer_bind(struct wl_client* client,
                               void* data,
                               uint32_t version,
                               uint32_t id)
{
    LOG_WAYL2("Binding Wayland pointer (version: %d, id: %d)", version, id);

    struct wl_resource* rc;
    rc = wl_resource_create(client, &wl_pointer_interface, version, id);
    NOIA_ENSURE(rc, wl_client_post_no_memory(client); return);

    wl_resource_set_implementation(rc, &scPointerImplementation,
                                   data, noia_wayland_pointer_unbind);

    noia_wayland_facade_add_general_resource(NOIA_RESOURCE_POINTER, rc);
}

//------------------------------------------------------------------------------

